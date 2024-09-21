use clap::Parser;

fn main() {
    let config = catr::Config::parse();

    if let Err(e) = catr::run(config) {
        eprintln!("Ops, something went wrong while running the application: {e}");
        std::process::exit(1);
    };
}
