use clap::Parser;

fn main() {
    let config = grepr::Config::parse();

    if let Err(e) = grepr::run(config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
