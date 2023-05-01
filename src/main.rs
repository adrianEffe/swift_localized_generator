mod parser;
mod io;

use std::env;

fn main() {
    let read_file_path = env::args().nth(1).expect("Please provide a file path");
    let content = io::read(&read_file_path);
    let keys = parser::parse(&content);
    dbg!(keys);

    let file_path = env::args().nth(2).expect("Please provide a file path for LocalizedKeys.swift");
    io::write("keys placeholder", &file_path);
}
