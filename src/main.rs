mod utils;
mod brainfuck;

use anyhow::Result;

use clap::Parser;
use utils::file_utils::read_brainfuck_files;
use brainfuck::brainfuck_interpreter;

use crate::utils::cli::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    let code = read_brainfuck_files(&args.path)?;

    brainfuck_interpreter(&code)?;
    
    Ok(())
}
