use anyhow::Result;
use super::compiler::compile;
use super::runtime::run;

pub fn brainfuck_interpreter(code: Vec<u8>, input: Option<Vec<u8>>) -> Result<()> {
    compile(&code).and_then(|instructions| run(&instructions, input))
}