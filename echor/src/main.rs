use clap::Parser;

fn main() {
    let config = Config::parse();
    let output = config.text.join(" ");

    if config.no_newline {
        print!("{output}");
    } else {
        println!("{output}");
    }
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct Config {
    #[arg(required = true)]
    text: Vec<String>,

    #[arg(short, long)]
    no_newline: bool,
}
