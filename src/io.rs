use std::fs;
use std::fs::File;
use std::io::{ BufReader, Read};

pub fn read(file_path: &str) -> String {
    let file = File::open(file_path).expect("Failed to open file at given path");
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).expect("Failed to read to string");
    content
}

pub fn write(data: &str, file_path: &str){
    fs::write(file_path, data).expect("Failed to write to file");
}

#[cfg(test)]
mod test {
    use tempfile::tempdir;
    use super::*;

    #[test]
    fn test_io_write_read() {
        let dir = tempdir().unwrap();
        let file_path_buf = dir.path().join("my-temporary-note.txt");
        let path = file_path_buf.to_str().unwrap();
        write("data", path);
        let content = read(path);
        assert_eq!(content, "data");
    }
}
