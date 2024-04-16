// nfm-core: The core technologies behind No-Flavor Markdown.
// Copyright (C) 2024  Frankie Baffa
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! The core technologies behind No-Flavor Markdown.

#[cfg(test)]
mod test;

use std::{
    fs::OpenOptions,
    io::{
        Read,
        Result as IOResult,
    },
    path::Path,
    str::Lines,
};

#[derive(Debug)]
pub struct Parser<'a> {
    source: Lines<'a>,
    line: &'a str,
    in_paragraph: bool,
    in_pre_code: bool,
    in_blockquote: bool,
    in_table: bool,
    table_td_close: Option<String>,
    list_nest: Vec<String>,
    in_code_fence: bool,
    code_fence_first: bool,
    in_strong: bool,
    in_em: bool,
    in_del: bool,
    in_ins: bool,
    in_mark: bool,
    in_code: bool,
    in_sup: bool,
    output: String,
}

impl<'a> Parser<'a> {
    fn advance(&mut self, n: usize) {
        self.line = &self.line[n..];
    }

    fn consume(&mut self, n: usize) -> &str {
        let s = &self.line[0..n];
        self.line = &self.line[n..];
        s
    }

    fn advance_into(&mut self, n: usize, into: &mut String) {
        into.push_str(&self.line[0..n]);
        self.advance(n);
    }

    fn advance_into_output(&mut self, n: usize) {
        self.output.push_str(&self.line[0..n]);
        self.advance(n);
    }

    fn is_empty(&self) -> bool {
        self.line.is_empty()
    }

    fn split_off(&mut self, n: usize) -> String {
        let s = self.line[0..n].to_owned();
        self.line = &self.line[n..];
        s
    }

    fn advance_line(&mut self) -> bool {
        let l = self.source.next();
        if l.is_none() {
            self.line = "";
            return false;
        }

        self.line = l.unwrap();
        true
    }

    fn starts_with_number(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        matches!(&self.line[0..1], "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9")
    }

    fn starts_with_trimmed_char(&self, c: char) -> bool {
        self.line.trim_start_matches(' ').starts_with(c)
    }

    fn starts_with_trimmed(&self, s: &str) -> bool {
        self.line.trim_start_matches(' ').starts_with(s)
    }

    fn trim_start(&mut self) {
        self.line = self.line.trim_start_matches(' ');
    }

    fn in_list(&self) -> bool {
        !self.list_nest.is_empty()
    }

    fn list_level(&self) -> usize {
        self.list_nest.len()
    }

    fn count_indentation_levels(&mut self) -> usize {
        let mut i = 0;
        while self.line.starts_with("    ") {
            self.advance(4);
            i += 1;
        }

        self.trim_start();

        i
    }

    fn handle_list_level_discrepancy<'h>(&mut self, symbol: &'h str, open: &'h str, close: &'h str) {
        let lvl = self.count_indentation_levels() + 1;
        self.advance(symbol.len());
        let diff = self.list_level() as i32 - lvl as i32;

        match diff {
            // same
            0 => {},
            // list decreased in level
            1.. => {
                let diff = diff as usize;

                for _ in 0..diff {
                    self.output.push_str(&self.list_nest.pop().unwrap());
                }
            },
            // list increased in level
            _ => {
                let diff = (0 - diff) as usize;

                for _ in 0..diff {
                    self.output.push_str(open);
                    self.list_nest.push(close.to_owned());
                }
            }
        }
    }

    fn in_an_element(&mut self) -> bool {
        self.in_paragraph || self.in_pre_code || self.in_blockquote ||
            self.in_table || self.in_code_fence || self.in_list()
    }

    fn escape(&mut self, pattern: &str) -> bool {
        if self.line.starts_with(pattern) {
            self.advance(1);
            self.advance_into_output(pattern.len()-1);
            true
        } else {
            false
        }
    }

    fn br(&mut self) -> bool {
        if !self.line.eq("  ") {
            return false;
        }

        self.advance(2);
        self.output.push_str("<br />");
        true
    }

    fn strong(&mut self) -> bool {
        if self.line.starts_with("**") {
            self.advance(2);

            if self.in_strong {
                self.output.push_str("</strong>");
                self.in_strong = false;
            } else {
                self.output.push_str("<strong>");
                self.in_strong = true;
            }

            true
        } else {
            false
        }
    }

    fn em(&mut self) -> bool {
        if self.line.starts_with('_') {
            self.advance(1);
            if self.in_em {
                self.output.push_str("</em>");
                self.in_em = false;
            } else {
                self.output.push_str("<em>");
                self.in_em = true;
            }

            true
        } else {
            false
        }
    }

    fn del(&mut self) -> bool {
        if self.line.starts_with("~~") {
            self.advance(2);
            if self.in_del {
                self.output.push_str("</del>");
                self.in_del = false;
            } else {
                self.output.push_str("<del>");
                self.in_del = true;
            }

            true
        } else {
            false
        }
    }

    fn ins(&mut self) -> bool {
        if self.line.starts_with("++") {
            self.advance(2);
            if self.in_ins {
                self.output.push_str("</ins>");
                self.in_ins = false;
            } else {
                self.output.push_str("<ins>");
                self.in_ins = true;
            }

            true
        } else {
            false
        }
    }

    fn mark(&mut self) -> bool {
        if self.line.starts_with("==") {
            self.advance(2);
            if self.in_mark {
                self.output.push_str("</mark>");
                self.in_mark = false;
            } else {
                self.output.push_str("<mark>");
                self.in_mark = true;
            }

            true
        } else {
            false
        }
    }

    fn code(&mut self) -> bool {
        if self.line.starts_with('`') {
            self.advance(1);
            if self.in_code {
                self.output.push_str("</code>");
                self.in_code = false;
            } else {
                self.output.push_str("<code>");
                self.in_code = true;
            }

            true
        } else {
            false
        }
    }

    fn sup(&mut self) -> bool {
        if self.line.starts_with('^') {
            self.advance(1);
            if self.in_sup {
                self.output.push_str("</sup>");
                self.in_sup = false;
            } else {
                self.output.push_str("<sup>");
                self.in_sup = true;
            }

            true
        } else {
            false
        }
    }

    fn anchor(&mut self) -> bool {
        if self.line.starts_with('<') {
            self.advance(1);

            let idx = match self.line.find('>') {
                None => {
                    self.output.push_str("&lt;");
                    return true;
                },
                Some(idx) => idx,
            };

            let link = self.split_off(idx);

            self.advance(1);
            self.output.push_str("<a id=\"");
            self.output.push_str(&link);
            self.output.push_str("\"></a>");

            true
        } else {
            false
        }
    }

    fn unchecked(&mut self) -> bool {
        if self.line.starts_with("[ ]") {
            self.advance(3);

            self.output.push_str("<input type=\"checkbox\" disabled=\"disabled\" />");

            true
        } else {
            false
        }
    }

    fn checked(&mut self) -> bool {
        if self.line.starts_with("[x]") {
            self.advance(3);

            self.output.push_str("<input type=\"checkbox\" disabled=\"disabled\" checked=\"checked\" />");

            true
        } else {
            false
        }
    }

    fn link(&mut self) -> bool {
        if self.line.starts_with('[') {
            let end_idx = match self.line.find(']') {
                None => {
                    self.advance_into_output(1);
                    return true;
                },
                Some(idx) => {
                    self.advance(1);
                    idx - 1
                },
            };

            let mut inner = String::new();
            std::mem::swap(&mut self.output, &mut inner);

            let mut line = &self.line[0..end_idx];
            self.advance(end_idx);
            std::mem::swap(&mut self.line, &mut line);

            self.parse_inline();

            std::mem::swap(&mut self.output, &mut inner);
            std::mem::swap(&mut self.line, &mut line);

            self.advance(1);

            if !self.line.starts_with('(') {
                self.output.push('[');
                self.output.push_str(&inner);
                self.output.push(']');
            } else {
                self.advance(1);

                let idx = match self.line.find(')') {
                    None => {
                        self.output.push('[');
                        self.output.push_str(&inner);
                        self.output.push(']');
                        self.output.push('(');
                        return true;
                    },
                    Some(idx) => idx,
                };

                let href = self.split_off(idx);
                self.advance(1);

                self.output.push_str("<a href=\"");
                self.output.push_str(&href);
                self.output.push_str("\">");
                self.output.push_str(&inner);
                self.output.push_str("</a>");
            }

            true
        } else {
            false
        }
    }

    fn img(&mut self) -> bool {
        if self.line.starts_with("![") {
            self.advance(2);

            let idx = match self.line.find(']') {
                None => {
                    self.output.push_str("![");
                    return true;
                },
                Some(idx) => idx,
            };

            let alt_text = self.split_off(idx);
            self.advance(1);

            if !self.line.starts_with('(') {
                self.output.push_str("![");
                self.output.push_str(&alt_text);
                self.output.push(']');
                return true;
            }

            self.advance(1);

            let idx = match self.line.find(')') {
                None => {
                    self.output.push_str("![");
                    self.output.push_str(&alt_text);
                    self.output.push(']');
                    self.output.push('(');
                    return true;
                },
                Some(idx) => idx,
            };

            let src = self.split_off(idx);
            self.advance(1);
            self.output.push_str("<img alt=\"");
            self.output.push_str(&alt_text);
            self.output.push_str("\" src=\"");
            self.output.push_str(&src);
            self.output.push_str("\" />");

            true
        } else {
            false
        }
    }

    fn table(&mut self) -> bool {
        if self.line.starts_with('|') && self.in_table {
            self.advance(1);

            match &self.table_td_close {
                Some(close) => {
                    self.output.push_str(close);
                    self.table_td_close = None;
                },
                None => {},
            }

            // ELEMENT/SCOPE
            let mut element = "td";
            let mut scope = "";
            // column header
            if self.line.starts_with('=') {
                self.advance(1);
                element = "th";
                scope = "col";
            }
            // row header
            else if self.line.starts_with('-') {
                self.advance(1);
                element = "th";
                scope = "row";
            }
            // standard td (default. used only for alignment of flags)
            else if self.line.starts_with('_') {
                self.advance(1);
            }

            // ALIGN
            let mut align = "center";
            // right alignment
            if self.line.starts_with('$') {
                self.advance(1);
                align = "right";
            }
            // left alignment
            else if self.line.starts_with('^') {
                self.advance(1);
                align = "left";
            }
            // center alignment (default. used only for alignment of flags)
            else if self.line.starts_with('_') {
                self.advance(1);
            }

            // VALIGN
            let mut valign = "baseline";
            // top valignment
            if self.line.starts_with('t') {
                self.advance(1);
                valign = "top";
            }
            // middle valignment
            else if self.line.starts_with('m') {
                self.advance(1);
                valign = "middle";
            }
            // bottom valignment
            else if self.line.starts_with('b') {
                self.advance(1);
                valign = "bottom";
            }
            // baseline valignment (default. used only for alignment of flags)
            else if self.line.starts_with('_') {
                self.advance(1);
            }

            // COLSPAN
            let mut colspan = "1".to_owned();
            if self.line.starts_with('_') || self.line.starts_with('0') || self.starts_with_number() {
                colspan.clear();

                loop {
                    if self.is_empty() {
                        break;
                    }
                    else if (self.line.starts_with('_') || self.line.starts_with('0')) && colspan.is_empty() {
                        self.advance(1);
                        continue;
                    }
                    else if self.starts_with_number() {
                        self.advance_into(1, &mut colspan);
                        continue;
                    }
                    else {
                        break;
                    }
                }

                // if empty, revert to default
                if colspan.is_empty() {
                    colspan.push('1');
                }
            }

            // ROWSPAN
            let mut rowspan = "1".to_owned();
            if self.line.starts_with(',') {
                self.advance(1);

                if self.line.starts_with('_') || self.line.starts_with('0') || self.starts_with_number() {
                    rowspan.clear();

                    loop {
                        if self.is_empty() {
                            break;
                        }
                        else if (self.line.starts_with('_') || self.line.starts_with('0')) && rowspan.is_empty() {
                            self.advance(1);
                            continue;
                        }
                        else if self.starts_with_number() {
                            self.advance_into(1, &mut rowspan);
                            continue;
                        }
                        else {
                            break;
                        }
                    }

                    // if empty, revert to default
                    if rowspan.is_empty() {
                        rowspan.push('1');
                    }
                }
            }

            self.trim_start();

            if element == "th" {
                self.output.push_str("<th scope=\"");
                self.output.push_str(scope);
                self.output.push_str("\" align=\"");
                self.output.push_str(align);
                self.output.push_str("\" valign=\"");
                self.output.push_str(&valign);
                self.output.push_str("\" colspan=\"");
                self.output.push_str(&colspan);
                self.output.push_str("\" rowspan=\"");
                self.output.push_str(&rowspan);
                self.output.push_str("\">");
                self.table_td_close = Some("</th>".to_owned());
            } else {
                self.output.push_str("<td align=\"");
                self.output.push_str(align);
                self.output.push_str("\" valign=\"");
                self.output.push_str(&valign);
                self.output.push_str("\" colspan=\"");
                self.output.push_str(&colspan);
                self.output.push_str("\" rowspan=\"");
                self.output.push_str(&rowspan);
                self.output.push_str("\">");
                self.table_td_close = Some("</td>".to_owned());
            }

            true
        } else {
            false
        }
    }

    fn parse_inline(&mut self) {
        while !self.is_empty() {
            if self.escape("\\\\") ||
                // escape strong
                self.escape("\\*") ||
                // escape em
                self.escape("\\_") ||
                // escape del
                self.escape("\\~") ||
                // escape ins
                self.escape("\\+") ||
                // escape mark
                self.escape("\\=") ||
                // escape code
                self.escape("\\`") ||
                // escape sup
                self.escape("\\^") ||
                // escape link
                self.escape("\\[") ||
                // escape image
                self.escape("\\![") ||
                // escape table
                self.escape("\\|") {}
            // escape link
            else if self.line.starts_with("\\<") {
                self.advance(2);
                // encode html
                self.output.push_str("&lt;");
            }
            else if !self.in_code && (
                // br
                self.br() ||
                // strong
                self.strong() ||
                // em
                self.em() ||
                // del
                self.del() ||
                // ins
                self.ins() ||
                // mark
                self.mark() ||
                // superscript
                self.sup() ||
                // anchor
                self.anchor() ||
                // checkbox unchecked
                self.unchecked() ||
                // checkbox checked
                self.checked() ||
                // link
                self.link() ||
                // image
                self.img() ||
                // table
                self.table()
            ) ||
                // code
                self.code() {}
            // text
            else {
                // trim end padding in tables
                if self.in_table && self.starts_with_trimmed_char('|') {
                    self.trim_start();
                } else {
                    let character = match self.consume(1) {
                        "<" => "&lt;",
                        ">" => "&gt;",
                        c => c,
                    }.to_owned();
                    self.output.push_str(&character);
                }
            }
        }
    }

    fn parse(&mut self) {
        macro_rules! revert_paragraph {
            () => {
                if self.in_paragraph {
                    self.output.push_str("</p>\n");
                    self.in_paragraph = false;
                }
            }
        }

        macro_rules! revert_pre_code {
            () => {
                if self.in_pre_code {
                    self.output.push_str("</code></pre>\n");
                    self.in_pre_code = false;
                }
            }
        }

        macro_rules! revert_list {
            () => {
                if self.in_list() {
                    self.output.push_str("</li>");
                    while self.in_list() {
                        self.output.push_str(&self.list_nest.pop().unwrap());
                    }
                    self.output.push_str("\n");
                }
            }
        }

        macro_rules! revert_blockquote {
            () => {
                if self.in_blockquote {
                    self.output.push_str("</blockquote>\n");
                    self.in_blockquote = false;
                }
            }
        }

        macro_rules! revert_table {
            () => {
                if self.in_table {
                    match &self.table_td_close {
                        Some(close) => {
                            self.output.push_str(close);
                            self.table_td_close = None;
                        },
                        None => {},
                    }
                    self.output.push_str("</tr></tbody></table>\n");
                    self.in_table = false;
                }
            }
        }

        macro_rules! revert_code_fence {
            () => {
                if self.in_code_fence {
                    self.output.push_str("</code></pre>\n");
                    self.in_code_fence = false;
                    self.code_fence_first = false;
                }
            }
        }

        macro_rules! revert_strong {
            () => {
                if self.in_strong {
                    self.output.push_str("</strong>");
                    self.in_strong = false;
                }
            }
        }

        macro_rules! revert_em {
            () => {
                if self.in_em {
                    self.output.push_str("</em>");
                    self.in_em = false;
                }
            }
        }

        macro_rules! revert_del {
            () => {
                if self.in_del {
                    self.output.push_str("</del>");
                    self.in_del = false;
                }
            }
        }

        macro_rules! revert_ins {
            () => {
                if self.in_ins {
                    self.output.push_str("</ins>");
                    self.in_ins = false;
                }
            }
        }

        macro_rules! revert_mark {
            () => {
                if self.in_mark {
                    self.output.push_str("</mark>");
                    self.in_mark = false;
                }
            }
        }

        macro_rules! revert_all_but_block {
            () => {
                // all inline modifiers must come first
                revert_strong!();
                revert_em!();
                revert_del!();
                revert_ins!();
                revert_mark!();
            }
        }

        macro_rules! revert_all_but_paragraph {
            () => {
                revert_all_but_block!();
                revert_list!();
                revert_pre_code!();
                revert_blockquote!();
                revert_table!();
            }
        }

        macro_rules! revert_all_but_list {
            () => {
                revert_all_but_block!();
                revert_paragraph!();
                revert_pre_code!();
                revert_blockquote!();
                revert_table!();
            }
        }

        macro_rules! revert_all_but_pre_code {
            () => {
                revert_all_but_block!();
                revert_paragraph!();
                revert_list!();
                revert_blockquote!();
                revert_table!();
            }
        }

        macro_rules! revert_all_but_blockquote {
            () => {
                revert_all_but_block!();
                revert_paragraph!();
                revert_list!();
                revert_pre_code!();
                revert_table!();
            }
        }

        macro_rules! revert_all_but_table {
            () => {
                revert_all_but_block!();
                revert_paragraph!();
                revert_list!();
                revert_pre_code!();
                revert_blockquote!();
            }
        }

        macro_rules! revert_all {
            () => {
                revert_all_but_block!();
                revert_paragraph!();
                revert_list!();
                revert_pre_code!();
                revert_blockquote!();
                revert_table!();
            }
        }

        loop {
            if !self.advance_line() {
                break;
            }

            // end element
            if self.is_empty() {
                revert_all!();
                self.output.push('\n');
                continue;
            }
            // h6
            else if !self.in_an_element() && self.line.starts_with("######") {
                revert_all!();
                self.advance(6);
                self.output.push_str("<h6>");
                self.trim_start();
                self.parse_inline();
                self.output.push_str("</h6>\n");
                continue;
            }
            // h5
            else if !self.in_an_element() && self.line.starts_with("#####") {
                revert_all!();
                self.advance(5);
                self.output.push_str("<h5>");
                self.trim_start();
                self.parse_inline();
                self.output.push_str("</h5>\n");
                continue;
            }
            // hr
            else if !self.in_an_element() && self.line.eq("- - -") {
                revert_all!();
                self.advance(5);
                self.output.push_str("<hr />\n");
                continue;
            }
            // h4
            else if !self.in_an_element() && self.line.starts_with("####") {
                revert_all!();
                self.advance(4);
                self.output.push_str("<h4>");
                self.trim_start();
                self.parse_inline();
                self.output.push_str("</h4>\n");
                continue;
            }
            // ul
            else if !self.in_pre_code && !self.in_paragraph && !self.in_blockquote && !self.in_table && !self.in_code_fence && self.starts_with_trimmed_char('-') {
                revert_all_but_list!();
                if self.in_list() {
                    self.output.push_str("</li>");
                }
                self.handle_list_level_discrepancy("-", "<ul>", "</ul>");
                self.output.push_str("<li>");
                self.trim_start();
                self.parse_inline();
                continue;
            }
            // ol
            else if !self.in_pre_code && !self.in_paragraph && !self.in_blockquote && !self.in_table && !self.in_code_fence && self.starts_with_trimmed("0.") {
                revert_all_but_list!();
                if self.in_list() {
                    self.output.push_str("</li>");
                }
                self.handle_list_level_discrepancy("0.", "<ol>", "</ol>");
                self.output.push_str("<li>");
                self.trim_start();
                self.parse_inline();
                continue;
            }
            // pre-code
            else if !self.in_paragraph && !self.in_blockquote && !self.in_table && !self.in_code_fence && !self.in_list() && self.line.starts_with("    ") {
                revert_all_but_pre_code!();
                self.advance(4);
                if !self.in_pre_code {
                    self.output.push_str("<pre><code>");
                    self.in_pre_code = true;
                } else {
                    self.output.push('\n');
                }

                // do NOT parse code blocks
                while !self.is_empty() {
                    // encode <, >, and space
                    let character = match &self.line[0..1] {
                        "<" => "&lt;",
                        ">" => "&gt;",
                        " " => "&nbsp;",
                        c => c,
                    };
                    self.output.push_str(character);
                    self.advance(1);
                }

                continue;
            }
            // code fence
            else if (
                !self.in_paragraph && !self.in_blockquote && !self.in_table &&
                !self.in_pre_code && !self.in_list() &&
                self.line.starts_with("```")
            ) || self.in_code_fence {
                // fence doesn't get reverted until ``` occurs again
                revert_all!();

                if !self.in_code_fence {
                    self.advance(3);
                    if !self.line.is_empty() {
                        let mut lang = String::new();
                        while !self.line.is_empty() {
                            lang.push_str(match &self.line[0..1] {
                                "<" => "&lt;",
                                ">" => "&gt;",
                                "\"" => "",
                                "\'" => "",
                                c => c,
                            });
                            self.advance(1);
                        }

                        self.output.push_str("<pre><code lang=\"");
                        self.output.push_str(&lang);
                        self.output.push_str("\">");
                    } else {
                        self.output.push_str("<pre><code>");
                    }
                    self.in_code_fence = true;
                    self.code_fence_first = true;
                    continue;
                } else if self.line.starts_with("```") {
                    self.advance(3);
                    revert_code_fence!();
                    continue;
                }

                if !self.code_fence_first {
                    self.output.push('\n');
                } else {
                    self.code_fence_first = false;
                }

                while !self.line.is_empty() && !self.line.starts_with("```") {
                    if self.line.starts_with("\\`") {
                        self.advance(1);
                    }
                    // encode <, >, and space
                    self.output.push_str(match &self.line[0..1] {
                        "<" => "&lt;",
                        ">" => "&gt;",
                        " " => "&nbsp;",
                        c => c,
                    });
                    self.advance(1);
                }

                continue;
            }
            // h3
            else if !self.in_an_element() && self.line.starts_with("###") {
                revert_all!();
                self.advance(3);
                self.output.push_str("<h3>");
                self.trim_start();
                self.parse_inline();
                self.output.push_str("</h3>\n");
                continue;
            }
            // h2
            else if !self.in_an_element() && self.line.starts_with("##") {
                revert_all!();
                self.advance(2);
                self.output.push_str("<h2>");
                self.trim_start();
                self.parse_inline();
                self.output.push_str("</h2>\n");
                continue;
            }
            // h1
            else if !self.in_an_element() && self.line.starts_with('#') {
                revert_all!();
                self.advance(1);
                self.output.push_str("<h1>");
                self.trim_start();
                self.parse_inline();
                self.output.push_str("</h1>\n");
                continue;
            }
            // blockquote
            else if !self.in_pre_code && !self.in_paragraph && !self.in_table && !self.in_code_fence && !self.in_list() && self.line.starts_with('>') {
                self.advance(1);
                if !self.in_blockquote {
                    revert_all_but_blockquote!();
                    self.output.push_str("<blockquote>");
                    self.in_blockquote = true;
                } else {
                    self.output.push('\n');
                }

                if !self.br() {
                    self.trim_start();
                }

                self.parse_inline();
                continue;
            }
            // table
            else if (
                !self.in_pre_code && !self.in_paragraph && !self.in_blockquote &&
                !self.in_code_fence && !self.in_list() && self.line.starts_with('|')
            ) || (
                self.in_table && self.starts_with_trimmed_char('|')
            ) {
                revert_all_but_table!();
                if !self.in_table {
                    self.output.push_str("<table><tbody><tr>");
                    self.in_table = true;
                } else {
                    match &self.table_td_close {
                        Some(close) => {
                            self.output.push_str(close);
                            self.table_td_close = None;
                        },
                        None => {},
                    }
                    self.output.push_str("</tr><tr>");
                }

                self.parse_inline();
                continue;
            }
            // if in a list and line is not empty, then still in same list-element
            else if self.in_list() {
                self.output.push('\n');
                self.trim_start();
                self.parse_inline();
                continue;
            }
            // p
            else if !self.in_paragraph {
                revert_all_but_paragraph!();
                self.output.push_str("<p>");
                self.in_paragraph = true;
            } else if self.in_paragraph {
                self.output.push('\n');
            }

            // escape all block level
            if self.line.starts_with("\\#") || self.line.starts_with("\\-") ||
                self.line.starts_with("\\>") || self.line.starts_with("\\0") ||
                self.line.starts_with("\\|") || self.line.starts_with("\\ ") ||
                self.line.starts_with("\\`")
            {
                self.advance(1);
            } else if self.line.starts_with("\\\\") {
                self.advance(1);
                self.advance_into_output(1);
            }

            self.parse_inline();
        }

        revert_all!();
        revert_code_fence!();
    }

    fn from_str(input: &'a str) -> Self {
        Self {
            source: input.lines(),
            line: "",
            in_paragraph: false,
            in_pre_code: false,
            in_blockquote: false,
            in_table: false,
            table_td_close: None,
            in_code_fence: false,
            code_fence_first: false,
            in_strong: false,
            in_em: false,
            in_del: false,
            in_ins: false,
            in_mark: false,
            in_code: false,
            in_sup: false,
            list_nest: Vec::new(),
            output: String::new(),
        }
    }

    pub fn parse_str(input: &'a str) -> String {
        let mut p = Self::from_str(input);
        p.parse();
        p.output
    }

    pub fn parse_file<P: AsRef<Path>>(path: P) -> IOResult<String> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let mut p = Parser::from_str(&s);
        p.parse();
        Ok(p.output)
    }
}
