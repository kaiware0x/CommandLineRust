use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .takes_value(false)
                .help("Prints number of lines (With blank lines)")
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .takes_value(false)
                .help("Prints number of lines (Without blank lines)"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank_lines");

    return Ok(Config {
        files: files,
        number_lines: number_lines,
        number_nonblank_lines: number_nonblank_lines,
    });
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(handle) => {
                let mut line_num = 0;

                for line in handle.lines() {
                    let line = line?;

                    if config.number_lines {
                        line_num += 1;
                        println!("{line_num:>6}\t{line}");
                    } else if config.number_nonblank_lines {
                        if line.trim().is_empty() {
                            println!("")
                        } else {
                            line_num += 1;
                            println!("{line_num:>6}\t{line}");
                        }
                    } else {
                        line_num += 1;
                        println!("{line}");
                    }
                }
            }
        }
    }
    return Ok(());
}
