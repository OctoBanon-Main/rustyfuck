use rustyfuck::{brainfuck, cli::parse_cli_arguments, src_reader};

fn main() {
    let args = match parse_cli_arguments() {
        Some(args) => args,
        None => return
    };

    match src_reader::read_brainfuck_files(&args.file_path) {
        Ok(code) => {
            let input_bytes = &args.input.map(|s| s.bytes().collect::<Vec<u8>>());
            brainfuck::brainfuck_interpreter(code, input_bytes.clone());
        },
        Err(error) => {
            eprintln!("File reading error: {}", error)
        }
    }
}
