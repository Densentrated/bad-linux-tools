#![allow(unused)]
#![allow(non_snake_case)]
use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Cli {
    filepath: std::path::PathBuf,
}

fn main() {

    let args: Cli = Cli::parse();
    let mut content: String = std::fs::read_to_string(&args.filepath).expect("could not read file");



    // replaces "l and r" characters with "w" characters
    content= content.replace("l", "w");
    content = content.replace("r", "w");
    
    //TODO switch from while to for loop"
    //TODO consolidate while loops

    let mut i: usize = 0;
    while i < content.len() {

        // replaces "n!" with "nya!"
        if (content.chars().nth(i) == Some('n') && content.chars().nth(i + 1) == Some('!')) {
            content.insert(i+1, 'y');
            content.insert(i+2, 'a')
        }
        
        // update iteraator varaible
        i += 1;
    
    }

    // prints the changed content
    println!("{}", content);   
}
