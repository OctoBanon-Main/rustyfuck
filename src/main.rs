use anyhow::{Result, anyhow};

use rustyfuck::{
    utils::{
        cli::parse_cli_arguments,
        file_utils::read_brainfuck_files
    },
    brainfuck::interpreter
};

fn main() -> Result<()> {
    let args = parse_cli_arguments()?
        .ok_or_else(|| anyhow!("No arguments provided"))?;

    let code = read_brainfuck_files(&args.file_path)?;
    let input_bytes = args.input.map(|s| s.bytes().collect::<Vec<u8>>());

    interpreter::brainfuck_interpreter(code, input_bytes.clone())?;

    Ok(())
}
