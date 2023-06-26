use fs;
use std::io::stdin;

fn main() {
    let fileName: str = std::io::stdin().read_line();
    let fileText: str = fs::read_to_string("filename").unwrap();
}