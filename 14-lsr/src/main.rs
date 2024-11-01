use clap::Parser;
use lsr::{run, Config};
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = run(config) {
        eprintln!("{}", e);
        process::exit(1);
    };
}
