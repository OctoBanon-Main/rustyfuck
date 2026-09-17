use anyhow::{Result, bail};
use std::{fs, path::Path};

pub fn read_brainfuck_files(file_path: &str) -> Result<Vec<u8>> {
    check_file_extension(file_path)?;

    let mut contents = fs::read(file_path)?;
    contents.retain(|&b| matches!(b, b'+' | b'-' | b'<' | b'>' | b'[' | b']' | b'.' | b','));

    if contents.is_empty() {
        bail!("File does not contain Brainfuck instructions.");
    }

    Ok(contents)
}

fn check_file_extension(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);

    match path.extension().and_then(|ext| ext.to_str()) {
        Some("b") | Some("bf") => Ok(()),
        _ => bail!("Only .b or .bf file extensions are supported."),
    }
}