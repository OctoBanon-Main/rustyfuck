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

    interpreter::brainfuck_interpreter(code)?;

    Ok(())
}
