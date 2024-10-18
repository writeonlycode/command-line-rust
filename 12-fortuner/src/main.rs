use clap::Parser;
use fortuner::Config;
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = fortuner::run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
