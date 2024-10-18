use anyhow::Result;
use clap::Parser;
use rand::{seq::SliceRandom, SeedableRng};
use regex::RegexBuilder;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use walkdir::WalkDir;

pub fn run(config: Config) -> Result<()> {
    let pattern = &config.pattern.map(|pattern| {
        match RegexBuilder::new(&pattern)
            .case_insensitive(config.insensitive)
            .build()
        {
            Ok(value) => value,
            Err(e) => panic!(
                "invalid value '{}' for '--pattern <PATTERN>': {}",
                pattern, e
            ),
        }
    });

    let files = find_files(&config.file)?;
    let fortunes = read_fortunes(&files)?;

    match pattern {
        Some(pattern) => {
            let filtered_fortunes = fortunes
                .into_iter()
                .filter(|e| pattern.is_match(e.text.as_str()));

            let mut previous_source = String::new();

            for fortune in filtered_fortunes {
                if previous_source != fortune.source {
                    previous_source = fortune.source.clone();
                    eprintln!(
                        "({})\n%",
                        PathBuf::from(fortune.source)
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                    );
                }

                print!("{}%\n", fortune.text);
            }
        }
        None => match pick_fortune(&fortunes, config.seed) {
            Some(chosen_fortune) => {
                print!("{}", chosen_fortune);
            }
            None => {
                println!("No fortunes found");
            }
        },
    }

    Ok(())
}

fn find_files(file_paths: &[String]) -> Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = Vec::new();

    for file_path in file_paths {
        let walker = WalkDir::new(file_path);
        let walker = walker
            .into_iter()
            .filter_entry(|a| !a.file_name().to_string_lossy().starts_with("."));

        for entry in walker {
            let entry = entry?.into_path();

            if !entry.is_dir() {
                result.push(entry);
            }
        }
    }

    result.sort();
    result.dedup();

    Ok(result)
}

fn read_fortunes(paths: &[PathBuf]) -> Result<Vec<Fortune>> {
    let mut result: Vec<Fortune> = vec![];

    for path in paths {
        let mut file = open_file(path.to_string_lossy().to_string().as_str())?;

        let mut line = String::new();
        let mut fortune_text = String::new();

        loop {
            let bytes = file.read_line(&mut line)?;

            if bytes == 0 {
                break;
            }

            if line.starts_with('%') && !fortune_text.is_empty() {
                result.push(Fortune {
                    source: path.to_string_lossy().to_string(),
                    text: fortune_text.to_string(),
                });
                fortune_text.clear();
            } else {
                fortune_text.push_str(line.as_str());
            }

            line.clear();
        }
    }

    Ok(result)
}

fn open_file(file_path: &str) -> Result<BufReader<File>> {
    Ok(BufReader::new(File::open(file_path)?))
}

fn pick_fortune(fortunes: &[Fortune], seed: Option<u64>) -> Option<String> {
    match seed {
        Some(seed) => {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let chosen_fortune = fortunes.choose(&mut rng);
            chosen_fortune.map(|value| value.text.clone())
        }
        None => {
            let mut rng = rand::thread_rng();
            let chosen_fortune = fortunes.choose(&mut rng);
            chosen_fortune.map(|value| value.text.clone())
        }
    }
}

#[derive(Debug, Clone, Parser)]
#[command(about, version)]
pub struct Config {
    #[arg(required = true, help = "A list of input files or directories")]
    pub file: Vec<String>,

    #[arg(short = 'm', long, help = "A pattern to filter fortunes")]
    pub pattern: Option<String>,

    #[arg(short, long, help = "A seed to control random selections")]
    pub seed: Option<u64>,

    #[arg(short, long, help = "Case-insensitive pattern matching")]
    pub insensitive: bool,
}

#[derive(Debug)]
struct Fortune {
    source: String,
    text: String,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;

    use crate::pick_fortune;

    use super::{find_files, read_fortunes, Fortune};

    #[test]
    fn test_find_files() {
        // Success on existent file
        let result = find_files(&["./tests/inputs/jokes".to_string()]);
        assert!(result.is_ok());

        let files = result.unwrap();
        assert_eq!(files.len(), 1);

        let first_file = files.get(0).unwrap();
        assert_eq!(first_file.to_string_lossy(), "./tests/inputs/jokes");

        // Fail on non-existent file
        let result = find_files(&["./tests/inputs/non-existent-jokes".to_string()]);
        assert!(result.is_err());

        // Check number and order of files
        let result = find_files(&[
            "./tests/inputs/jokes".to_string(),
            "./tests/inputs/ascii-art".to_string(),
            "./tests/inputs/jokes".to_string(),
        ]);
        assert!(result.is_ok());

        let files = result.unwrap();
        assert_eq!(files.len(), 2);

        if let Some(filename) = files.first().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "ascii-art".to_string());
        }

        if let Some(filename) = files.last().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "jokes".to_string());
        }
    }

    #[test]
    fn test_read_fortunes() {
        // One input file
        let result = read_fortunes(&[PathBuf::from("./tests/inputs/jokes")]);
        assert!(result.is_ok());

        if let Ok(fortunes) = result {
            // Correct number and sorting
            assert_eq!(fortunes.len(), 6);
            assert_eq!(
                fortunes.first().unwrap().text,
                "Q. What do you call a head of lettuce in a shirt and tie?\nA. Collared greens.\n"
            );
            assert_eq!(
                fortunes.last().unwrap().text,
                "Q: What do you call a deer wearing an eye patch?\nA: A bad idea (bad-eye deer).\n"
            );
        }

        // Multiple files
        let result = read_fortunes(&[
            PathBuf::from("./tests/inputs/jokes"),
            PathBuf::from("./tests/inputs/quotes"),
        ]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 11);
    }

    #[test]
    fn test_pick_fortune() {
        let fortunes = &[
            Fortune {
                source: "fortunes".to_string(),
                text: "You cannot achieve the impossible without attempting the absurd."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Assumption is the mother of all screw-ups.".to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Neckties strangle clear thinking.".to_string(),
            },
        ];

        // Pick a fortune with a seed
        assert_eq!(
            pick_fortune(fortunes, Some(1)).unwrap(),
            "Neckties strangle clear thinking.".to_string()
        );
    }
}
