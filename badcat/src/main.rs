#![allow(unused)]
use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Cli {
    filepath: std::path::PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
    let mut content: String = std::fs::read_to_string(&args.filepath).expect("could not read file");
    let kaomoji_list: Vec<&str> = vec!["(o^▽^o)", "<(￣︶￣)>", "ヽ(・∀・)ﾉ", "(o･ω･o)", "(o´▽`o)", "(*´▽`*)", "(o´∀`o)"] ;

    // replaces "l and r" characters with "w" characters
    content= content.replace("l", "w");
    content = content.replace("r", "w");
    
    //TODO switch from while to for loop"
    //TODO consolidate while loops

    let mut i: usize = 0;
    while i < content.len() {
        // replaces "th" with "ff"
        if (content.chars().nth(i) == Some('t')){
            if (content.chars().nth(i+1) == Some('h') || content.chars().nth(i+1) == Some('H')){
                content.replace_range(i..i+2, "ff");
            }
        }

        // replaces "n!" with "nya!"
        if (content.chars().nth(i) == Some('n') && content.chars().nth(i + 1) == Some('!')) {
            content.insert(i+1, 'y');
            content.insert(i+2, 'a')
        }

        // inserts random kaomoji into the text
        if content.chars().nth(i) == Some(' ') {
            let random_number: i32 = rand::thread_rng().gen_range(0..20);
        }
        
        // update iteraator varaible
        i += 1;
    
    }

    // prints the changed content
    println!("{}", content);   
}
