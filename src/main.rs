mod formatter;
mod io;
mod parser;

use std::env;

fn main() {
    let read_file_path = env::args().nth(1).expect("Please provide a file path");
    let content = io::read(&read_file_path);
    let keys = parser::parse(&content);

    let swift_file = formatter::format(&keys);

    let file_path = env::args()
        .nth(2)
        .expect("Please provide a file path for LocalizationKeys.swift");
    io::write(&swift_file, &file_path);
}
