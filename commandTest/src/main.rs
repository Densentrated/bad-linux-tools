use clap::Parser;

struct Cli {
    _pattern: String,
    _file_path: std::path::PathBuf,
}

fn main() {
    println!("Hello, world!");
    let _pattern = std::env::args().nth(1).expect("no pattern given");
    let _file_path = std::env::args().nth(2).expect("no filepath given");
    let _arugments = Cli {
        _pattern: _pattern,
        _file_path: std::path::PathBuf::from(_file_path),
    };
}


