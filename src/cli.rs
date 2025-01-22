use std::{env, path::Path};

pub struct CliArguments {
    pub file_path: String,
    pub input: Option<String>
}

pub fn parse_cli_arguments() -> Option<CliArguments> {
    // Getting CLI arguments
    let args: Vec<String> = env::args().collect();

    // Check for version flag (-v or --version)
    if args.len() > 1 && (args[1] == "-v" || args[1] == "--version") {
        let version = env!("CARGO_PKG_VERSION");
        println!("Rustyfuck interpreter, version: {}", version);
        return None;
    }

    if args.len() < 2 {
        eprintln!("Error: not enough arguments. Usage: <brainfuck src file> [input (optional)]");
        return None;
    }

    // Getting file path argument
    let file_path = args[1].clone();

    // Checking extension
    let path = Path::new(&file_path);
    if let Some(extension) = path.extension() {
        if extension != "b" && extension != "bf" {
            eprintln!("Error: Only .b or .bf file extensions are supported.");
            return None;
        }
    } else {
        eprintln!("Error: Cannot find file extension.");
        return None;
    }
    
    // Getting optional input
    let input = if args.len() > 2 {
        Some(args[2].clone())
    } else {
        None
    };

    Some(CliArguments { file_path, input } )
}