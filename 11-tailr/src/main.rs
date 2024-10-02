use clap::Parser;
use tailr::{run, Config};

fn main() {
    let config = Config::parse();

    if let Err(e) = run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
