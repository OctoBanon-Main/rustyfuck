mod instructions;
mod compiler;
mod runtime;

use anyhow::Result;
use super::brainfuck::compiler::compile;
use super::brainfuck::runtime::run;

pub fn brainfuck_interpreter(code: &[u8]) -> Result<()> {
    compile(code).and_then(|instructions| run(&instructions))
}