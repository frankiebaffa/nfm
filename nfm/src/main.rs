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
    clap::Parser as Cli,
    nfm_core::Parser,
    std::{
        fs::OpenOptions,
        io::{ Read, Result, stdin, Write, },
        process::exit,
        time::Instant,
    },
};

const LICENSE_NOTICE: &str = include_str!("../../NOTICE-GPL");
const LICENSE_FULL: &str = include_str!("../../LICENSE-GPL");

/// No-Flavor Markdown - Convert markdown to html.
#[derive(Cli)]
struct Args {
    /// Print timing information.
    #[arg(short, long)]
    timing: bool,
    /// Do not print output or save to file.
    #[arg(short='n', long="dry-run")]
    dry_run: bool,
    /// Output to a file.
    #[arg(short, long)]
    output_path: Option<String>,
    /// Read from stdin
    #[arg(short='i', long="read-stdin")]
    read_stdin: bool,
    /// Print the license notice.
    #[arg(short='l')]
    license_notice: bool,
    /// Print the license in full.
    #[arg(short='L')]
    license_full: bool,
    /// The path to the file to parse.
    path: Option<String>,
}

fn main() -> Result<()> {
    let Args {
        timing, dry_run, output_path, path, read_stdin, license_notice,
        license_full
    } = Args::parse();

    if license_full {
        println!("{LICENSE_FULL}");
        return Ok(());
    } else if license_notice {
        println!("{LICENSE_NOTICE}");
        return Ok(());
    }

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
