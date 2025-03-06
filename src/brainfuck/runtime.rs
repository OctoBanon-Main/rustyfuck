use anyhow::Result;
use super::instructions::Instruction;

pub fn run(instructions: &[Instruction], input: Option<Vec<u8>>) -> Result<()> {
    const MEMORY_SIZE: usize = 30000;
    let mut memory = [0u8; MEMORY_SIZE];
    let mut pointer: usize = 0;
    let mut ip: usize = 0;
    let mut input_pointer = 0;
    let input_slice = input.as_deref();

    while ip < instructions.len() {
        match instructions[ip] {
            Instruction::Move(offset) => {
                pointer = (((pointer as isize) + offset).rem_euclid(MEMORY_SIZE as isize)) as usize;
            },
            Instruction::Add(delta) => {
                memory[pointer] = memory[pointer].wrapping_add(delta as u8);
            },
            Instruction::Output => {
                print!("{}", memory[pointer] as char);
            },
            Instruction::Input => {
                memory[pointer] = input_slice.and_then(|bytes| bytes.get(input_pointer))
                    .copied().unwrap_or(0);
                input_pointer += if memory[pointer] != 0 { 1 } else { 0 };
            },
            Instruction::JumpIfZero(jump) if memory[pointer] == 0 => {
                ip = jump;
                continue;
            },
            Instruction::JumpIfNonZero(jump) if memory[pointer] != 0 => {
                ip = jump;
                continue;
            },
            _ => {},
        }
        ip += 1;
    }
    Ok(())
}

