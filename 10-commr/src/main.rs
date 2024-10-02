use clap::Parser;
use commr::run;

fn main() {
    let config = commr::Config::parse().validate();

    if let Err(e) = run(config) {
        eprintln!("Error while running the application: {}", e);
        std::process::exit(1);
    }
}
