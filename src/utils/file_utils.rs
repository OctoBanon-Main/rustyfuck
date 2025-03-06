use std::{fs, io::{self, BufReader, Read}, path::Path};
use anyhow::{bail, Result};

pub fn read_brainfuck_files(file_path: &str) -> io::Result<Vec<u8>> {
    let file = fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    let mut byte = [0u8; 1];
    while let Ok(_) = reader.read_exact(&mut byte) {
        if matches!(byte[0], b'+' | b'-' | b'<' | b'>' | b'[' | b']' | b'.' | b',') {
            buffer.push(byte[0]);
        }
    }

    Ok(buffer)
}

pub fn check_file_extension(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);

    match path.extension().and_then(|ext| ext.to_str()) {
        Some("b") | Some("bf") => Ok(()),
        _ => bail!("Only .b or .bf file extensions are supported.")
    }
}