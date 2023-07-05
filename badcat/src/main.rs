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
    content= content.replace("l", "w");
    content = content.replace("r", "w");
    
    //TODO switch from while to for loop"
    
    // replaces "th" with "ff"
    let mut i = 0;
    while i < content.len() {
        if (content.chars().nth(i) == Some('t')){
            if (content.chars().nth(i+1) == Some('h') || content.chars().nth(i+1) == Some('H')){
                content.replace_range(i..i+2, "ff");
            }
        }
        i += 1;
    }

    //TODO switch form while to for loop"
    let mut i = 0;
    while i < content.len() {
        if (content.chars().nth(i) == Some('n') && content.chars().nth(i + 1) == Some('!')) {
            content.insert(i+1, 'y');
            content.insert(i+2, 'a')
        }
        i += 1;
    }



/**
    for char in content.chars(){
        if (char == n) {
            content.insert(,y);
        }
    }
 **/

    // prints the changed content
    println!("{}", content);   
}
