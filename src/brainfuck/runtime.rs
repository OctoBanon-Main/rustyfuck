use anyhow::Result;
use std::io::{self, Read, Write};

use super::instructions::Instruction;

pub fn run(instructions: &[Instruction]) -> Result<()> {
    const MEMORY_SIZE: usize = 30000;
    let mut memory = [0u8; MEMORY_SIZE];
    let mut pointer: usize = 0;
    let mut ip: usize = 0;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    while ip < instructions.len() {
        match instructions[ip] {
            Instruction::Move(offset) => {
                pointer = (pointer.wrapping_add(offset as usize)) % MEMORY_SIZE;
            },
            Instruction::Add(delta) => {
                memory[pointer] = memory[pointer].wrapping_add(delta as u8);
            },
            Instruction::Output => {
                handle.write_all(&[memory[pointer]])?;
            },
            Instruction::OutputN(n) => {
                let buf = vec![memory[pointer]; n];
                handle.write_all(&buf)?;
            },
            Instruction::Input => {
                memory[pointer] = io::stdin().bytes()
                    .next()
                    .and_then(|bytes| bytes.ok())
                    .unwrap_or(0);
            },
            Instruction::JumpIfZero(jump) if memory[pointer] == 0 => {
                ip = jump;
                continue;
            },
            Instruction::JumpIfNonZero(jump) if memory[pointer] != 0 => {
                ip = jump;
                continue;
            },
            Instruction::ClearCell => {
                memory[pointer] = 0;
            },
            _ => {},
        }
        ip += 1;
    }
    handle.flush()?;
    Ok(())
}

