#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    filepath: std::path::PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
    let mut content: String = std::fs::read_to_string(&args.filepath).expect("could not read file");

    // replaces "l and r" characters with "w" characters
    let content: String = content.replace("l", "w");
    let content: String = content.replace("r", "w");
    println!("{}", content);

    // replaces "th" with "ff"
    
}
