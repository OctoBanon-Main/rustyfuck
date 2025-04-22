use std::{fs, io::{self, Read}, path::Path};
use anyhow::{bail, Result};

pub fn read_brainfuck_files(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let brainfuck_bytes = contents
        .into_iter()
        .filter(|&b| matches!(b, b'+' | b'-' | b'<' | b'>' | b'[' | b']' | b'.' | b','))
        .collect();

    Ok(brainfuck_bytes)
}

pub fn check_file_extension(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);

    match path.extension().and_then(|ext| ext.to_str()) {
        Some("b") | Some("bf") => Ok(()),
        _ => bail!("Only .b or .bf file extensions are supported.")
    }
}