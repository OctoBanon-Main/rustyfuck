use std::env;
use clap::{Command, Arg};
use anyhow::Result;

use super::file_utils::check_file_extension;

pub struct CliArguments {
    pub file_path: String,
}

pub fn parse_cli_arguments() -> Result<Option<CliArguments>> {
    let matches = Command::new("Rustyfuck Interpreter")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Maksim Bengraf")
        .about("Interprets Brainfuck source files")
        .arg_required_else_help(true)
        .arg(
            Arg::new("path")
                .help("Path to the Brainfuck source file")
                .required(true)
                .index(1)
        )
        .get_matches();


    let file_path = matches.get_one::<String>("path").unwrap().clone();
    check_file_extension(&file_path)?;

    Ok(Some(CliArguments { file_path }))
}