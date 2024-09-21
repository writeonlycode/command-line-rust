use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run(config: Config) -> Result<()> {
    let mut line_number = 0;

    for file in config.files.iter() {
        match open(file) {
            Ok(file) => {
                for line in file.lines() {
                    let line = line?;
                    if config.number_lines {
                        line_number += 1;
                        println!("{:6}\t{line}", line_number);
                    } else if config.number_nonblank_lines && !line.is_empty() {
                        line_number += 1;
                        println!("{:6}\t{line}", line_number);
                    } else {
                        println!("{line}");
                    }
                }
            }
            Err(error) => {
                eprintln!("{file}: {error}");
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(filename)?))),
    }
}

#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub struct Config {
    #[arg(default_value = "-", help = "Input file(s)")]
    pub files: Vec<String>,

    #[arg(
        short = 'n',
        long = "number-nonblank",
        help = "Number all output lines"
    )]
    pub number_lines: bool,

    #[arg(
        short = 'b',
        long = "number",
        help = "Number nonempty output lines, overrides -n"
    )]
    pub number_nonblank_lines: bool,
}
