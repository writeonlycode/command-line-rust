use std::{
    fs::{metadata, File},
    io::{stdin, BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;
use regex::{Regex, RegexBuilder};

pub fn run(config: Config) -> Result<()> {
    let pattern = RegexBuilder::new(&config.pattern)
        .case_insensitive(config.ignore_case)
        .build()?;

    let file_names = Vec::from(config.files);

    for file_name in &file_names {
        if config.recursive && metadata(file_name)?.is_dir() {
            for file_name in walkdir::WalkDir::new(file_name) {
                let file_name = file_name?.path().to_string_lossy().to_string();
                let mut file = open_file(&file_name)?;

                if config.count {
                    let _ = print_number_of_matches_in_file(
                        &pattern,
                        &mut file,
                        format!("{}:", file_name),
                    );
                } else {
                    let _ = print_matches_in_file(&pattern, &mut file, format!("{}:", file_name));
                }
            }
        } else {
            match open_file(&file_name) {
                Ok(mut file) => {
                    if let Ok(metadata) = metadata(&file_name) {
                        if metadata.is_dir() {
                            eprintln!("{} is a directory", &file_name);
                        }
                    }

                    if file_names.len() > 1 {
                        if config.count {
                            let _ = print_number_of_matches_in_file(
                                &pattern,
                                &mut file,
                                format!("{}:", file_name),
                            );
                        } else {
                            let _ = print_matches_in_file(
                                &pattern,
                                &mut file,
                                format!("{}:", file_name),
                            );
                        }
                    } else {
                        if config.count {
                            let _ =
                                print_number_of_matches_in_file(&pattern, &mut file, format!(""));
                        } else {
                            let _ = print_matches_in_file(&pattern, &mut file, format!(""));
                        }
                    };
                }
                Err(e) => {
                    eprintln!("{}: {}", file_name, e);
                }
            }
        }
    }

    Ok(())
}

fn open_file(file_name: &str) -> Result<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

fn print_matches_in_file(r: &Regex, f: &mut Box<dyn BufRead>, p: String) -> Result<()> {
    let mut line = String::new();

    loop {
        let bytes = f.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        if r.is_match(&line) {
            print!("{}{}", p, line);
        }

        line.clear();
    }

    Ok(())
}

fn print_number_of_matches_in_file(r: &Regex, f: &mut Box<dyn BufRead>, p: String) -> Result<()> {
    let result = f.lines().fold(0, |e, l| {
        let l = l.unwrap();

        if r.is_match(&l) {
            return e + 1;
        } else {
            return e;
        }
    });

    println!("{}{}", p, result);
    Ok(())
}

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Config {
    #[arg(help = "PATTERN for matching")]
    pattern: String,

    #[arg(default_value = "-", help = "Input FILE(s)")]
    files: Vec<String>,

    #[arg(short, long = "insensitive", help = "Case insensitive matching")]
    ignore_case: bool,

    #[arg(short, long, help = "Print the number of lines that match")]
    count: bool,

    #[arg(short, long, help = "Search recursively inside directories")]
    recursive: bool,
}
