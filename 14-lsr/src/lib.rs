use anyhow::Result;
use chrono::{DateTime, Local};
use clap::Parser;
use std::{
    fs::{metadata, read_dir},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};
use tabular::{Row, Table};
use users::{get_group_by_gid, get_user_by_uid};

pub fn run(config: Config) -> Result<()> {
    if config.long {
        let mut table = Table::new("{:<}{:<} {:<} {:<} {:<} {:>} {:<} {:<}");

        for file in find_files(config.files, config.all)? {
            let mt = file.metadata()?;
            let md = mt.mode();

            let ur = if (md & (0x1 << 8)) >= 1 { "r" } else { "-" };
            let uw = if (md & (0x1 << 7)) >= 1 { "w" } else { "-" };
            let ux = if (md & (0x1 << 6)) >= 1 { "x" } else { "-" };
            let gr = if (md & (0x1 << 5)) >= 1 { "r" } else { "-" };
            let gw = if (md & (0x1 << 4)) >= 1 { "w" } else { "-" };
            let gx = if (md & (0x1 << 3)) >= 1 { "x" } else { "-" };
            let or = if (md & (0x1 << 2)) >= 1 { "r" } else { "-" };
            let ow = if (md & (0x1 << 1)) >= 1 { "w" } else { "-" };
            let ox = if (md & 0x1) >= 1 { "x" } else { "-" };

            let user_owner = get_user_by_uid(mt.uid()).unwrap();
            let group_owner = get_group_by_gid(mt.gid()).unwrap();

            let modified: DateTime<Local> = DateTime::from(mt.modified().unwrap());

            table.add_row(
                Row::new()
                    .with_cell(if mt.is_dir() { "d" } else { "-" })
                    .with_cell(format!(
                        "{}{}{}{}{}{}{}{}{}",
                        ur, uw, ux, gr, gw, gx, or, ow, ox
                    ))
                    .with_cell(mt.nlink())
                    .with_cell(user_owner.name().to_string_lossy().to_string())
                    .with_cell(group_owner.name().to_string_lossy().to_string())
                    .with_cell(mt.size())
                    .with_cell(modified.format("%b %d %y %H:%M"))
                    .with_cell(file.display()),
            );
        }

        print!("{}", table);
    } else {
        for file in find_files(config.files, config.all)? {
            println!("{}", file.display());
        }
    }

    Ok(())
}

fn find_files(paths: Vec<String>, all: bool) -> Result<Vec<PathBuf>> {
    let mut file_list = vec![];

    for path in paths {
        match metadata(&path) {
            Ok(mt) => {
                if mt.is_dir() {
                    let files = read_dir(&path)?;

                    for file in files {
                        let file_name = file?.file_name().to_string_lossy().to_string();

                        if !file_name.starts_with(".") || all {
                            let path_buffer: PathBuf = [&path, &file_name].iter().collect();
                            file_list.push(path_buffer);
                        }
                    }
                } else if mt.is_file() {
                    let path_buffer = PathBuf::from(&path);
                    file_list.push(path_buffer);
                } else if mt.is_symlink() {
                    let file = PathBuf::from(&path);
                    file_list.push(file);
                }
            }
            Err(e) => {
                eprintln!("{}: {}", path, e);
            }
        };
    }

    Ok(file_list)
}

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Config {
    #[arg(default_value = ".", help = "Files or folders to list")]
    files: Vec<String>,

    #[arg(short, long, help = "Show hidden files")]
    all: bool,

    #[arg(short, long, help = "Use long listing format")]
    long: bool,
}
