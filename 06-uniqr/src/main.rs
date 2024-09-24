use clap::Parser;

fn main() {
    let config = uniqr::Config::parse();

    if let Err(e) = uniqr::run(config) {
        eprint!("Ops, something went wrong while running the application: {e}");
        std::process::exit(1);
    }
}
