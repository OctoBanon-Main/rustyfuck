use std::{fs, io};

pub fn read_brainfuck_files(file_path: &str) -> io::Result<Vec<u8>> {
    let contents = fs::read(file_path)?;

    let filtered: Vec<u8> = contents
        .iter()
        .filter(|&&b| matches!(b, b'+' | b'-' | b'<' | b'>' | b'[' | b']' | b'.' | b','))
        .cloned()
        .collect();

    Ok(filtered)
}