use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Range,
};

use clap::{Arg, Parser};

pub fn run(config: Config) -> anyhow::Result<()> {
    for file in config.files {
        match open_file(&file) {
            Ok(file) => {
                for line in file.lines() {
                    let line = line?;
                    if let Some(ranges) = &config.bytes {
                        print_bytes_in_range(line.clone(), ranges)?;
                    };

                    if let Some(ranges) = &config.chars {
                        print_chars_in_range(line.clone(), ranges)?;
                    };

                    if let Some(ranges) = &config.fields {
                        print_fields_in_range(line.clone(), ranges, config.delim)?;
                    };
                }
            }
            Err(e) => {
                eprintln!("{}: {}", file, e)
            }
        }
    }
    Ok(())
}

fn open_file(file_name: &str) -> Result<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

fn print_bytes_in_range(s: String, ranges: &Vec<Range<usize>>) -> Result<()> {
    for range in ranges {
        let value: Vec<u8> = s
            .bytes()
            .skip(range.start - 1)
            .take(range.end - range.start + 1)
            .collect();
        print!("{}", String::from_utf8_lossy(value.as_slice()));
    }

    print!("\n");
    Ok(())
}

fn print_chars_in_range(s: String, ranges: &Vec<Range<usize>>) -> Result<()> {
    for range in ranges {
        let value: String = s
            .chars()
            .skip(range.start - 1)
            .take(range.end - range.start + 1)
            .collect();
        print!("{}", value.to_string());
    }

    print!("\n");
    Ok(())
}

fn print_fields_in_range(s: String, ranges: &Vec<Range<usize>>, delim: char) -> Result<()> {
    for range in ranges {
        let value: String = s
            .split(delim)
            .skip(range.start - 1)
            .take(range.end - range.start + 1)
            .fold(String::from(""), |a, e| format!("{}{}{}", a, e, delim));
        print!("{}", value.to_string().trim_end_matches(delim));
    }

    print!("\n");
    Ok(())
}

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Config {
    #[arg(default_value = "-", help = "Input FILE(s)")]
    pub files: Vec<String>,

    #[arg(short, long, default_value = "\t", help = "Field delimiter")]
    pub delim: char,

    #[arg(short, long, group = "fbc", required = true, help = "Selected fields", value_parser= parse_range)]
    fields: Option<Vec<Range<usize>>>,

    #[arg(short, long, group = "fbc", required = true, help = "Selected bytes", value_parser=parse_range)]
    bytes: Option<Vec<Range<usize>>>,

    #[arg(short, long, group = "fbc", required = true, help = "Selected chars", value_parser=parse_range)]
    chars: Option<Vec<Range<usize>>>,
}

fn parse_range(input: &str) -> Result<Range<usize>> {
    let parts: Vec<&str> = input.split('-').collect();

    if parts.len() == 1 {
        let start = parts[0].parse::<usize>()?;
        return Ok(Range { start, end: start });
    }

    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Error"));
    }

    let start = parts[0].parse::<usize>()?;
    let end = parts[1].parse::<usize>()?;

    if start > end {
        return Err(anyhow::anyhow!("Error"));
    }

    Ok(Range { start, end })
}
