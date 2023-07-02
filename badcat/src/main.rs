#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    param: String,
    filePath: std::path::PathBuf,
}

fn main(){
    println!("Hello World!")
}