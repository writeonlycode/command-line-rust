use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run(config: Config) -> Result<()> {
    let mut total_num_of_lines = 0;
    let mut total_num_of_words = 0;
    let mut total_num_of_chars = 0;
    let mut total_num_of_bytes = 0;

    let num_of_files = config.files.len();

    for filename in config.files {
        match open_file(&filename) {
            Ok(mut file) => {
                let (num_of_lines, num_of_words, num_of_chars, num_of_bytes) = count(&mut file)?;

                total_num_of_lines += num_of_lines;
                total_num_of_words += num_of_words;
                total_num_of_chars += num_of_chars;
                total_num_of_bytes += num_of_bytes;

                let num_of_lines = if config.lines {
                    format!("{:8}", num_of_lines)
                } else {
                    "".to_string()
                };

                let num_of_words = if config.words {
                    format!("{:8}", num_of_words)
                } else {
                    "".to_string()
                };

                let num_of_chars = if config.chars {
                    format!("{:8}", num_of_chars)
                } else {
                    "".to_string()
                };

                let num_of_bytes = if config.bytes {
                    format!("{:8}", num_of_bytes)
                } else {
                    "".to_string()
                };

                let filename = if filename != "-" {
                    format!(" {}", filename)
                } else {
                    "".to_string()
                };

                println!(
                    "{}{}{}{}{}",
                    num_of_lines, num_of_words, num_of_chars, num_of_bytes, filename
                )
            }
            Err(error) => {
                eprintln!("{filename}: {error}");
            }
        }
    }

    if num_of_files > 1 {
        let total_num_of_lines = if config.lines {
            format!("{:8}", total_num_of_lines)
        } else {
            "".to_string()
        };

        let total_num_of_words = if config.words {
            format!("{:8}", total_num_of_words)
        } else {
            "".to_string()
        };

        let total_num_of_chars = if config.chars {
            format!("{:8}", total_num_of_chars)
        } else {
            "".to_string()
        };

        let total_num_of_bytes = if config.bytes {
            format!("{:8}", total_num_of_bytes)
        } else {
            "".to_string()
        };

        println!(
            "{}{}{}{} {}",
            total_num_of_lines, total_num_of_words, total_num_of_chars, total_num_of_bytes, "total"
        )
    }

    Ok(())
}

fn open_file(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(&filename)?))),
    }
}

fn count(mut file: impl BufRead) -> Result<(usize, usize, usize, usize)> {
    let mut number_of_lines = 0;
    let mut number_of_words = 0;
    let mut number_of_chars = 0;
    let mut number_of_bytes = 0;

    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        number_of_lines += 1;
        number_of_bytes += bytes;
        number_of_words += line.split_whitespace().count();
        number_of_chars += line.chars().count();
        line.clear();
    }

    Ok((
        number_of_lines,
        number_of_words,
        number_of_chars,
        number_of_bytes,
    ))
}

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Config {
    #[arg(help = "Input file(s)", default_value = "-")]
    pub files: Vec<String>,

    #[arg(short = 'l', long = "lines", help = "Print the line count")]
    pub lines: bool,

    #[arg(short = 'w', long = "words", help = "Print the word count")]
    pub words: bool,

    #[arg(
        short = 'm',
        long = "chars",
        help = "Print the character count",
        group = "mc"
    )]
    pub chars: bool,

    #[arg(
        short = 'c',
        long = "bytes",
        help = "Print the byte count",
        group = "mc"
    )]
    pub bytes: bool,
}

#[cfg(test)]
mod tests {
    use super::count;
    use pretty_assertions::assert_eq;

    #[test]
    fn count_works() {
        let text = "The quick brown fox jumps over the lazy dog.";
        let mut cursor = std::io::Cursor::new(text);
        let result = count(&mut cursor).unwrap();
        let expected = (1, 9, 44, 44);

        assert_eq!(result, expected)
    }
}
