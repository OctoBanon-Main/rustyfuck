use std::env;
use clap::{Command, Arg};
use anyhow::Result;

use super::file_utils::check_file_extension;

pub struct CliArguments {
    pub file_path: String,
    pub input: Option<String>
}

pub fn parse_cli_arguments() -> Result<Option<CliArguments>> {
    let matches = Command::new("Rustyfuck Interpreter")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Maksim Bengraf")
        .about("Interprets Brainfuck source files")
        .arg_required_else_help(true)
        .arg(
            Arg::new("file_path")
                .help("The Brainfuck source file to interpret")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("input")
                .help("Optional input string for the Brainfuck program")
                .required(false)
                .index(2)
        )
        .get_matches();


    let file_path = matches.get_one::<String>("file_path").unwrap().clone();
    check_file_extension(&file_path)?;

    let input = matches.get_one::<String>("input").cloned();

    Ok(Some(CliArguments { file_path, input }))
}