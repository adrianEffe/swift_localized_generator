mod parser;

use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let keys = parser::parse(&content);
    dbg!(keys);
}
