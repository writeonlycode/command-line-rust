use regex::Regex;

pub fn run(config: Config) -> anyhow::Result<()> {
    for path in &config.paths {
        for entry in walkdir::WalkDir::new(path) {
            match entry {
                Ok(entry) => {
                    if filter_type(&entry, &config.entry_types)
                        && filter_name(&entry, &config.entry_names)
                    {
                        println!("{}", entry.path().display());
                    }
                }
                Err(e) => {
                    eprint!("({}): {}", path, e)
                }
            }
        }
    }
    Ok(())
}

fn filter_type(entry: &walkdir::DirEntry, entry_types: &Vec<EntryType>) -> bool {
    entry_types.is_empty()
        || entry_types.iter().any(|e| match e {
            EntryType::Dir => entry.file_type().is_dir(),
            EntryType::File => entry.file_type().is_file(),
            EntryType::Link => entry.file_type().is_symlink(),
        })
}

fn filter_name(entry: &walkdir::DirEntry, entry_names: &Vec<Regex>) -> bool {
    entry_names.is_empty()
        || entry_names.iter().any(|r| {
            let re = regex::Regex::new(r.as_str());
            re.unwrap().is_match(entry.file_name().to_str().unwrap())
        })
}

#[derive(Debug, clap::Parser)]
#[command(about, version)]
pub struct Config {
    #[arg(default_value = ".")]
    paths: Vec<String>,

    #[arg(name="NAME", short = 'n', long = "name", help = "Names to look for", num_args=0..)]
    entry_names: Vec<Regex>,

    #[arg(name="TYPE", short = 't', long = "type", help = "Types to look for", num_args=0..)]
    entry_types: Vec<EntryType>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug)]
enum EntryType {
    #[clap(name = "d")]
    Dir,

    #[clap(name = "f")]
    File,

    #[clap(name = "l")]
    Link,
}
