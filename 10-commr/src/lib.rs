use anyhow::{bail, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

pub fn run(config: Config) -> Result<()> {
    let mut lines1 = open_file(&config.file_1)?.lines().filter_map(Result::ok);
    let mut lines2 = open_file(&config.file_2)?.lines().filter_map(Result::ok);

    let mut current_line_1 = lines1.next();
    let mut current_line_2 = lines2.next();

    loop {
        match (current_line_1.clone(), current_line_2.clone()) {
            (Some(line1), Some(line2)) => {
                let mut iline1 = line1.clone();
                let mut iline2 = line2.clone();

                if config.insensitive {
                    iline1 = line1.to_lowercase();
                    iline2 = line2.to_lowercase();
                }
                if iline1 == iline2 {
                    print_col_3(line1.to_string(), &config);
                    current_line_1 = lines1.next();
                    current_line_2 = lines2.next();
                } else if iline1 > iline2 {
                    print_col_2(line2.to_string(), &config);
                    current_line_2 = lines2.next();
                } else {
                    print_col_1(line1.to_string(), &config);
                    current_line_1 = lines1.next();
                }
            }
            (Some(line1), None) => {
                print_col_1(line1.to_string(), &config);
                current_line_1 = lines1.next();
            }
            (None, Some(line2)) => {
                print_col_2(line2.to_string(), &config);
                current_line_2 = lines2.next();
            }
            (None, None) => break,
        }
    }

    Ok(())
}

fn print_col_1(line: String, config: &Config) {
    if !config.supress_col_1 {
        println!("{}", line);
    }
}
fn print_col_2(line: String, config: &Config) {
    let del = config.delimiter.clone();

    if !config.supress_col_2 {
        println!("{}{}", if !config.supress_col_1 { &del } else { "" }, line);
    }
}
fn print_col_3(line: String, config: &Config) {
    let del = config.delimiter.clone();

    if !config.supress_col_3 {
        println!(
            "{}{}{}",
            if !config.supress_col_1 { &del } else { "" },
            if !config.supress_col_2 { &del } else { "" },
            line
        );
    }
}

#[derive(Debug, Parser)]
pub struct Config {
    #[arg(name = "FILE1", default_value = "-", help = "Input FILE")]
    file_1: String,

    #[arg(name = "FILE2", default_value = "-", help = "Input FILE")]
    file_2: String,

    #[arg(short = '1', help = "Supress column 1 (lines unique to FILE1)")]
    supress_col_1: bool,
    #[arg(short = '2', help = "Supress column 2 (lines unique to FILE2)")]
    supress_col_2: bool,
    #[arg(
        short = '3',
        help = "Supress column 3 (lines in that appear in both files)"
    )]
    supress_col_3: bool,

    #[arg(short = 'i', help = "Case insensitive comparison of lines")]
    insensitive: bool,

    #[arg(
        default_value = "\t",
        short = 'd',
        help = "Case insensitive comparison of lines"
    )]
    delimiter: String,
}

impl Config {
    pub fn validate(self) -> Config {
        if self.file_1 == "-" && self.file_2 == "-" {
            eprintln!("error: Both input files cannot be STDIN (\"-\")\n");
            eprintln!("Usage: commr <FILE1> [FILE2]\n");
            eprintln!("For more information, try '--help'.");
            std::process::exit(1);
        }

        self
    }
}

fn open_file(file_name: &str) -> Result<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => match File::open(file_name) {
            Ok(file) => Ok(Box::new(BufReader::new(file))),
            Err(e) => bail!("{}: {}", file_name, e),
        },
    }
}
