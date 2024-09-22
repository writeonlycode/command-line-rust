use clap::Parser;

fn main() {
    let mut config = wcr::Config::parse();

    if !config.lines && !config.words && !config.chars && !config.bytes {
        config.lines = true;
        config.words = true;
        config.bytes = true;
    }

    if let Err(e) = wcr::run(config) {
        eprintln!("Ops, something went wrong while running the application: {e}");
        std::process::exit(1);
    }
}
