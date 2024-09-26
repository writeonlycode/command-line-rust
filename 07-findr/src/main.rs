use clap::Parser;

fn main() {
    let config = findr::Config::parse();

    if let Err(e) = findr::run(config) {
        println!("Error: {e}");
        std::process::exit(1);
    }
}
