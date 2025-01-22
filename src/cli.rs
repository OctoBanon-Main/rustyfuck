use std::{env, path::Path};

pub struct CliArguments {
    pub file_path: String,
    pub input: Option<String>
}

pub fn parse_cli_arguments() -> Option<CliArguments> {
    // Getting CLI arguments
    let args: Vec<String> = env::args().collect();

    // Print usage if not having arguments
    if args.len() < 2 {
        let program_name = args[0]
            .rsplit_once(std::path::MAIN_SEPARATOR)
            .map(|(_, name)| name)
            .unwrap_or(&args[0]);

        println!("Usage: {} <brainfuck src file> [input(optional)]", program_name);
        return None;
    }

    // Check for version flag (-v or --version)
    if args.len() > 1 && (args[1] == "-v" || args[1] == "--version") {
        let version = env!("CARGO_PKG_VERSION");
        println!("Rustyfuck interpreter, version: {}", version);
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