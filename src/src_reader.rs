use std::{fs::File, io::{self, Read}};

pub fn read_brainfuck_files(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}