// nfm-core::test: The primary tests for No-Flavor Markdown.
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

//! The primary tests for No-Flavor Markdown.

use crate::Parser;

#[test]
fn anchor_1() {
    let html = Parser::parse_str(include_str!("../tests/anchor/1/test.md"));
    assert_eq!(include_str!("../tests/anchor/1/against.html"), html);
}

#[test]
fn br_1() {
    let html = Parser::parse_str(include_str!("../tests/br/1/test.md"));
    assert_eq!(include_str!("../tests/br/1/against.html"), html);
}

#[test]
fn br_2() {
    let html = Parser::parse_str(include_str!("../tests/br/2/test.md"));
    assert_eq!(include_str!("../tests/br/2/against.html"), html);
}

#[test]
fn blockquote_1() {
    let html = Parser::parse_str(include_str!("../tests/blockquote/1/test.md"));
    assert_eq!(include_str!("../tests/blockquote/1/against.html"), html);
}

#[test]
fn blockquote_2() {
    let html = Parser::parse_str(include_str!("../tests/blockquote/2/test.md"));
    assert_eq!(include_str!("../tests/blockquote/2/against.html"), html);
}

#[test]
fn checked_1() {
    let html = Parser::parse_str(include_str!("../tests/checked/1/test.md"));
    assert_eq!(include_str!("../tests/checked/1/against.html"), html);
}

#[test]
fn code_1() {
    let html = Parser::parse_str(include_str!("../tests/code/1/test.md"));
    assert_eq!(include_str!("../tests/code/1/against.html"), html);
}

#[test]
fn codefence_1() {
    let html = Parser::parse_str(include_str!("../tests/codefence/1/test.md"));
    assert_eq!(include_str!("../tests/codefence/1/against.html"), html);
}

#[test]
fn codefence_2() {
    let html = Parser::parse_str(include_str!("../tests/codefence/2/test.md"));
    assert_eq!(include_str!("../tests/codefence/2/against.html"), html);
}

#[test]
fn codefence_3() {
    let html = Parser::parse_str(include_str!("../tests/codefence/3/test.md"));
    assert_eq!(include_str!("../tests/codefence/3/against.html"), html);
}

#[test]
fn link_1() {
    let html = Parser::parse_str(include_str!("../tests/link/1/test.md"));
    assert_eq!(include_str!("../tests/link/1/against.html"), html);
}

#[test]
fn link_2() {
    let html = Parser::parse_str(include_str!("../tests/link/2/test.md"));
    assert_eq!(include_str!("../tests/link/2/against.html"), html);
}

#[test]
fn list_1() {
    let html = Parser::parse_str(include_str!("../tests/list/1/test.md"));
    assert_eq!(include_str!("../tests/list/1/against.html"), html);
}

#[test]
fn list_2() {
    let html = Parser::parse_str(include_str!("../tests/list/2/test.md"));
    assert_eq!(include_str!("../tests/list/2/against.html"), html);
}

#[test]
fn list_3() {
    let html = Parser::parse_str(include_str!("../tests/list/3/test.md"));
    assert_eq!(include_str!("../tests/list/3/against.html"), html);
}

#[test]
fn list_4() {
    let html = Parser::parse_str(include_str!("../tests/list/4/test.md"));
    assert_eq!(include_str!("../tests/list/4/against.html"), html);
}

#[test]
fn list_5() {
    let html = Parser::parse_str(include_str!("../tests/list/5/test.md"));
    assert_eq!(include_str!("../tests/list/5/against.html"), html);
}

#[test]
fn list_6() {
    let html = Parser::parse_str(include_str!("../tests/list/6/test.md"));
    assert_eq!(include_str!("../tests/list/6/against.html"), html);
}

#[test]
fn del_1() {
    let html = Parser::parse_str(include_str!("../tests/del/1/test.md"));
    assert_eq!(include_str!("../tests/del/1/against.html"), html);
}

#[test]
fn em_1() {
    let html = Parser::parse_str(include_str!("../tests/em/1/test.md"));
    assert_eq!(include_str!("../tests/em/1/against.html"), html);
}

#[test]
fn em_2() {
    let html = Parser::parse_str(include_str!("../tests/em/2/test.md"));
    assert_eq!(include_str!("../tests/em/2/against.html"), html);
}

#[test]
fn h1_1() {
    let html = Parser::parse_str(include_str!("../tests/h1/1/test.md"));
    assert_eq!(include_str!("../tests/h1/1/against.html"), html);
}

#[test]
fn h1_2() {
    let html = Parser::parse_str(include_str!("../tests/h1/2/test.md"));
    assert_eq!(include_str!("../tests/h1/2/against.html"), html);
}

#[test]
fn h2_1() {
    let html = Parser::parse_str(include_str!("../tests/h2/1/test.md"));
    assert_eq!(include_str!("../tests/h2/1/against.html"), html);
}

#[test]
fn h3_1() {
    let html = Parser::parse_str(include_str!("../tests/h3/1/test.md"));
    assert_eq!(include_str!("../tests/h3/1/against.html"), html);
}

#[test]
fn h4_1() {
    let html = Parser::parse_str(include_str!("../tests/h4/1/test.md"));
    assert_eq!(include_str!("../tests/h4/1/against.html"), html);
}

#[test]
fn h5_1() {
    let html = Parser::parse_str(include_str!("../tests/h5/1/test.md"));
    assert_eq!(include_str!("../tests/h5/1/against.html"), html);
}

#[test]
fn h6_1() {
    let html = Parser::parse_str(include_str!("../tests/h6/1/test.md"));
    assert_eq!(include_str!("../tests/h6/1/against.html"), html);
}

#[test]
fn hr_1() {
    let html = Parser::parse_str(include_str!("../tests/hr/1/test.md"));
    assert_eq!(include_str!("../tests/hr/1/against.html"), html);
}

#[test]
fn hr_2() {
    let html = Parser::parse_str(include_str!("../tests/hr/2/test.md"));
    assert_eq!(include_str!("../tests/hr/2/against.html"), html);
}

#[test]
fn img_1() {
    let html = Parser::parse_str(include_str!("../tests/img/1/test.md"));
    assert_eq!(include_str!("../tests/img/1/against.html"), html);
}

#[test]
fn ins_1() {
    let html = Parser::parse_str(include_str!("../tests/ins/1/test.md"));
    assert_eq!(include_str!("../tests/ins/1/against.html"), html);
}

#[test]
fn mark_1() {
    let html = Parser::parse_str(include_str!("../tests/mark/1/test.md"));
    assert_eq!(include_str!("../tests/mark/1/against.html"), html);
}

#[test]
fn p_1() {
    let html = Parser::parse_str(include_str!("../tests/p/1/test.md"));
    assert_eq!(include_str!("../tests/p/1/against.html"), html);
}

#[test]
fn p_2() {
    let html = Parser::parse_str(include_str!("../tests/p/2/test.md"));
    assert_eq!(include_str!("../tests/p/2/against.html"), html);
}

#[test]
fn p_3() {
    let html = Parser::parse_str(include_str!("../tests/p/3/test.md"));
    assert_eq!(include_str!("../tests/p/3/against.html"), html);
}

#[test]
fn p_4() {
    let html = Parser::parse_str(include_str!("../tests/p/4/test.md"));
    assert_eq!(include_str!("../tests/p/4/against.html"), html);
}

#[test]
fn p_5() {
    let html = Parser::parse_str(include_str!("../tests/p/5/test.md"));
    assert_eq!(include_str!("../tests/p/5/against.html"), html);
}

#[test]
fn p_6() {
    let html = Parser::parse_str(include_str!("../tests/p/6/test.md"));
    assert_eq!(include_str!("../tests/p/6/against.html"), html);
}

#[test]
fn precode_1() {
    let html = Parser::parse_str(include_str!("../tests/precode/1/test.md"));
    assert_eq!(include_str!("../tests/precode/1/against.html"), html);
}

#[test]
fn strong_1() {
    let html = Parser::parse_str(include_str!("../tests/strong/1/test.md"));
    assert_eq!(include_str!("../tests/strong/1/against.html"), html);
}

#[test]
fn strong_2() {
    let html = Parser::parse_str(include_str!("../tests/strong/2/test.md"));
    assert_eq!(include_str!("../tests/strong/2/against.html"), html);
}

#[test]
fn sup_1() {
    let html = Parser::parse_str(include_str!("../tests/sup/1/test.md"));
    assert_eq!(include_str!("../tests/sup/1/against.html"), html);
}

#[test]
fn unchecked_1() {
    let html = Parser::parse_str(include_str!("../tests/unchecked/1/test.md"));
    assert_eq!(include_str!("../tests/unchecked/1/against.html"), html);
}

#[test]
fn full_1() {
    let html = Parser::parse_str(include_str!("../tests/full/1/test.md"));
    assert_eq!(include_str!("../tests/full/1/against.html"), html);
}

#[test]
fn table_1() {
    let html = Parser::parse_str(include_str!("../tests/table/1/test.md"));
    assert_eq!(include_str!("../tests/table/1/against.html"), html);
}

#[test]
fn table_2() {
    let html = Parser::parse_str(include_str!("../tests/table/2/test.md"));
    assert_eq!(include_str!("../tests/table/2/against.html"), html);
}

#[test]
fn table_3() {
    let html = Parser::parse_str(include_str!("../tests/table/3/test.md"));
    assert_eq!(include_str!("../tests/table/3/against.html"), html);
}

#[test]
fn table_4() {
    let html = Parser::parse_str(include_str!("../tests/table/4/test.md"));
    assert_eq!(include_str!("../tests/table/4/against.html"), html);
}

#[test]
fn table_5() {
    let html = Parser::parse_str(include_str!("../tests/table/5/test.md"));
    assert_eq!(include_str!("../tests/table/5/against.html"), html);
}
