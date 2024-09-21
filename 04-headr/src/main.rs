use clap::Parser;

fn main() {
    let config = headr::Config::parse();

    if let Err(e) = headr::run(config) {
        eprintln!("Ops, something went wrong while running the application: {e}");
        std::process::exit(1);
    }
}
