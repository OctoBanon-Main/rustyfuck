use std::{env, path::Path};

use rustyfuck::{brainfuck, src_reader};

fn main() {
    // Getting CLI arguments
    let args: Vec<String> = env::args().collect();

    // Check for version flag (-v or --version)
    if args.len() > 1 && (args[1] == "-v" || args[1] == "--version") {
        let version = env!("CARGO_PKG_VERSION");
        println!("Rustyfuck interpreter, version: {}", version);
        return;
    }

    // Check argument length
    if args.len() < 2 {
        eprintln!("Error: not enough arguments. Usage: <brainfuck src file> [input (optional)]")
    }
    
    // Getting file path argument
    let file_path = &args[1];

    // Checking extension
    let path = Path::new(file_path);
    if let Some(extension) = path.extension() {
        if extension != "b" && extension != "bf" {
            eprintln!("Error: Only .b or .bf file extensions are supported.");
            return;
        }
    } else {
        eprintln!("Error: Cannot find file extension.");
        return;
    }

    // Getting optional input
    let input = if args.len() > 2 {
        Some(args[2].clone())
    } else {
        None
    };

    match src_reader::read_brainfuck_files(file_path) {
        Ok(code) => {
            let input_bytes = input.map(|s| s.bytes().collect::<Vec<u8>>());
            brainfuck::brainfuck_interpreter(code, input_bytes);
        },
        Err(error) => {
            eprintln!("File reading error: {}", error)
        }
    }
}
