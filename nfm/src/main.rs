// nfm: A parser for No-Flavor Markdown.
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

//! A parser for No-Flavor Markdown.

use {
    args::{ Arguments, OptionType, },
    nfm_core::Parser,
    std::{
        fs::OpenOptions,
        io::{ Error, ErrorKind, Read, Result, stdin, Write, },
        process::exit,
        time::Instant,
    },
};

const HELP: &str = include_str!("../resources/help.txt");
const LICENSE_NOTICE: &str = include_str!("../../NOTICE-GPL");
const LICENSE_FULL: &str = include_str!("../../LICENSE-GPL");

#[derive(Default)]
struct Args {
    timing: bool,
    dry_run: bool,
    output_path: Option<String>,
    read_stdin: bool,
    path: Option<String>,
}

fn main() -> Result<()> {
    let mut args = Args::default();
    Arguments::with_args(&mut args, |a, b, c| {
        match c.option_type() {
            OptionType::Argument(_) => match c.qualifier() {
                "h"|"help" => {
                    println!("{HELP}");
                    std::process::exit(0);
                },
                "t"|"timing" => b.timing = true,
                "n"|"dry-run" => b.dry_run = true,
                "o"|"output-path" => match a.next() {
                    Some(a) => match a.option_type() {
                        OptionType::Argument(_) => return Err(
                            Error::new(
                                ErrorKind::Other,
                                "-o|--output-path requires a value".to_owned(),
                            )
                        ),
                        OptionType::Value(_) => b.output_path = Some(a.qualifier().to_owned()),
                    },
                    None => return Err(Error::new(
                        ErrorKind::Other,
                        "-o|--output-path requires a value.".to_owned(),
                    )),
                },
                "i"|"read-stdin" => b.read_stdin = true,
                "l"|"license-notice" => {
                    println!("{LICENSE_NOTICE}");
                    std::process::exit(0);
                },
                "L"|"license-full" => {
                    println!("{LICENSE_FULL}");
                    std::process::exit(0);
                },
                q => return Err(Error::new(ErrorKind::Other, q.to_string())),
            },
            OptionType::Value(_) => if c.is_last() {
                b.path = Some(c.qualifier().to_owned());
            } else {
                return Err(
                    Error::new(
                        ErrorKind::Other,
                        "Value found in illegal position.".to_owned()
                    )
                );
            },
        }

        Ok(())
    })?;

    let Args { timing, dry_run, output_path, path, read_stdin } = args;

    let (output, dur) = if read_stdin {
            let stdin = stdin();
            let mut lock = stdin.lock();
            let mut input = String::new();
            lock.read_to_string(&mut input)?;

            if input.is_empty() {
                eprintln!("No data from stdin, argument PATH must be included");
                exit(2);
            }

            let start = if timing { Some(Instant::now()) } else { None };
            let output = Parser::parse_str(&input);
            let dur = if timing {
                Some((Instant::now() - start.unwrap()).as_nanos() as f64 / 1000000000_f64)
            } else {
                None
            };

            (output, dur)
    } else if let Some(path) = path {
        let start = if timing { Some(Instant::now()) } else { None };
        let output = Parser::parse_file(path)?;
        let dur = if timing {
            Some((Instant::now() - start.unwrap()).as_nanos() as f64 / 1000000000_f64)
        } else {
            None
        };

        (output, dur)
    } else {
        eprintln!("Argument path must be provided when not reading from stdin.");
        exit(2);
    };

    if !dry_run {
        match output_path {
            Some(path) => {
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)?;
                file.write_all(output.as_bytes())?;
            },
            None => {
                print!("{output}");
            },
        }
    }

    if timing {
        println!("{}s", dur.unwrap());
    }
    Ok(())
}
