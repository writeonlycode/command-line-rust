use std::fs::File;
use std::io::{BufRead, BufReader, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run(config: Config) -> Result<()> {
    let number_of_files = config.files.len();

    for (index, filename) in config.files.iter().enumerate() {
        match open_file(&filename) {
            Ok(mut file) => {
                if number_of_files > 1 {
                    println!("{}==> {} <==", if index != 0 { "\n" } else { "" }, filename);
                }

                match config.bytes {
                    Some(b) => {
                        let bytes: std::result::Result<Vec<_>, _> = file.bytes().take(b).collect();
                        print!("{}", String::from_utf8_lossy(&bytes?));
                    }
                    None => {
                        let lines = read_lines_from_file(&mut file, config.lines)?;
                        print!("{}", lines);
                    }
                }
            }
            Err(e) => eprintln!("{filename}: {e}"),
        }
    }

    Ok(())
}

fn open_file(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_lines_from_file(file: &mut Box<dyn BufRead>, number_of_lines: usize) -> Result<String> {
    let mut lines = String::new();

    for _ in 0..number_of_lines {
        let bytes_read = file.read_line(&mut lines)?;

        if bytes_read == 0 {
            break;
        }
    }

    Ok(lines)
}

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Config {
    #[arg(default_value = "-", help = "Input file(s)")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "lines",
        default_value = "10",
        group = "option",
        help = "The number of lines to print"
    )]
    lines: usize,

    #[arg(
        required = false,
        short = 'c',
        long = "bytes",
        group = "option",
        help = "The number of bytes to print"
    )]
    bytes: Option<usize>,
}
