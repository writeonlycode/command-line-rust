use std::io::{BufRead, Write};

use anyhow::{Ok, Result};
use clap::*;

pub fn run(config: Config) -> Result<()> {
    let mut file = open_file(config.input_file.as_str()).unwrap_or_else(|e| {
        eprintln!("{}: {}", config.input_file, e);
        std::process::exit(1);
    });

    let mut output = create_output(config.output_file.clone()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let mut current_line = String::new();
    let mut next_line = String::new();

    let bytes_read = file.read_line(&mut current_line)?;

    if bytes_read == 0 {
        return Ok(());
    }

    let mut counter = 1;

    loop {
        let bytes_read = file.read_line(&mut next_line)?;
        let mut count_string = String::new();

        if config.count {
            count_string = format!("{:4} ", counter);
        }

        if bytes_read == 0 {
            match config.output_file.clone() {
                Some(_) => {
                    let _ = output.write(format!("{}{}", count_string, current_line).as_bytes());
                }
                None => {
                    print!("{}{}", count_string, current_line);
                }
            };
            break;
        }

        if current_line.trim() != next_line.trim() {
            match config.output_file.clone() {
                Some(_) => {
                    let _ = output.write(format!("{}{}", count_string, current_line).as_bytes());
                }
                None => {
                    print!("{}{}", count_string, current_line);
                }
            };

            current_line = next_line.clone();
            counter = 1;
        } else {
            counter += 1;
        }

        next_line.clear();
    }

    Ok(())
}

fn open_file(file_path: &str) -> Result<Box<dyn BufRead>> {
    match file_path {
        "-" => Ok(Box::new(std::io::BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(std::io::BufReader::new(std::fs::File::open(
            file_path,
        )?))),
    }
}

fn create_output(file_path: Option<String>) -> Result<Box<dyn std::io::Write>> {
    match file_path {
        Some(file_path) => Ok(Box::new(std::fs::File::create(file_path.as_str())?)),
        None => Ok(Box::new(std::io::stdout())),
    }
}

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Config {
    #[arg(help = "Input file", default_value = "-")]
    input_file: String,

    #[arg(help = "Output file", requires = "input_file")]
    output_file: Option<String>,

    #[arg(
        short = 'c',
        long = "count",
        help = "Show the number of occurences before each line"
    )]
    count: bool,
}
