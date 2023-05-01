mod parser;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;


fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    let file = File::open(file_path).expect("Failed to open file at given path");
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).expect("Failed to read to string");

    let keys = parser::parse(&content);
    dbg!(keys);
}
