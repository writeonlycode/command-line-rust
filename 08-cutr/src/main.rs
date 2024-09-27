use clap::Parser;

fn main() {
    let config = cutr::Config::parse();

    if let Err(e) = cutr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
