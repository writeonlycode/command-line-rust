use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use anyhow::{anyhow, bail, Result};
use clap::Parser;

pub fn run(config: Config) -> Result<()> {
    let files = config.files.iter().enumerate();

    for (index, file_name) in files {
        if let Err(e) = open_file(&file_name) {
            eprintln!("{}: {}", file_name, e);
            continue;
        } else if config.files.len() > 1 && !config.supress_headers && index == 0 {
            println!("==> {} <==", file_name);
        } else if config.files.len() > 1 && !config.supress_headers {
            println!("\n==> {} <==", file_name);
        }

        let _ = match config.bytes {
            Some(number_of_bytes) => match number_of_bytes {
                Number::Positive(value) => print_bytes(&file_name, value, false),
                Number::Negative(value) => print_bytes(&file_name, value, true),
            },
            None => match config.lines {
                Number::Positive(value) => print_lines(&file_name, value, false),
                Number::Negative(value) => print_lines(&file_name, value, true),
            },
        };
    }

    Ok(())
}

fn open_file(file_name: &str) -> Result<BufReader<File>> {
    Ok(BufReader::new(File::open(file_name)?))
}

fn print_bytes(file_name: &str, number_of_bytes: usize, from_end: bool) -> Result<()> {
    let skip_bytes;

    if from_end {
        let file = open_file(file_name)?;
        let total_bytes = file.bytes().count();

        skip_bytes = if total_bytes < number_of_bytes {
            0
        } else {
            total_bytes - number_of_bytes
        };
    } else {
        skip_bytes = if number_of_bytes == 0 {
            0
        } else {
            number_of_bytes - 1
        };
    }

    let file = open_file(file_name)?;
    let bytes = file.bytes().skip(skip_bytes);

    let mut output: Vec<u8> = Vec::new();

    for byte in bytes {
        output.push(byte?);
    }

    print!("{}", String::from_utf8_lossy(&output));

    Ok(())
}

fn print_lines(file_name: &str, number_of_lines: usize, from_end: bool) -> Result<()> {
    let range;

    if from_end {
        let file = open_file(file_name)?;

        let total_lines = file.lines().count();
        let start_line = if total_lines < number_of_lines {
            0
        } else {
            total_lines - number_of_lines
        };

        range = 0..start_line;
    } else {
        range = 1..number_of_lines;
    }

    let mut file = open_file(file_name)?;
    let mut line = String::new();

    for _ in range {
        let bytes = file.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        line.clear();
    }

    loop {
        let bytes = file.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        print!("{}", line);
        line.clear();
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Config {
    #[arg(help = "Input files", required = true)]
    files: Vec<String>,

    #[arg(short = 'c', long, value_parser=parse_bytes, group="count", help = "Output the last number of bytes")]
    bytes: Option<Number>,

    #[arg(
        short = 'n',
        long,
        default_value = "10",
        value_parser=parse_lines,
        group="count",
        help = "Output the last number of lines"
    )]
    lines: Number,

    #[arg(short = 'q', long = "quiet", help = "Supress file headers")]
    supress_headers: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Positive(usize),
    Negative(usize),
}

fn parse_lines(input: &str) -> Result<Number> {
    let regex = regex::Regex::new(r"(?<sign>^[\+|\-]?)(?<value>.+)")?;

    let Some(m) = regex.captures(input) else {
        bail!("illegal line count -- {}", input);
    };

    let sign = &m["sign"];
    let value: usize = m["value"]
        .parse()
        .map_err(|e| anyhow!("illegal line count -- {}: {}", input, e))?;

    match sign {
        "+" => Ok(Number::Positive(value)),
        _ => Ok(Number::Negative(value)),
    }
}

fn parse_bytes(input: &str) -> Result<Number> {
    let regex = regex::Regex::new(r"(?<sign>^[\+|\-]?)(?<value>.+)")?;

    let Some(m) = regex.captures(input) else {
        bail!("illegal byte count -- {}", input);
    };

    let sign = &m["sign"];
    let value: usize = m["value"]
        .parse()
        .map_err(|e| anyhow!("illegal byte count -- {}: {}", input, e))?;

    match sign {
        "+" => Ok(Number::Positive(value)),
        _ => Ok(Number::Negative(value)),
    }
}
