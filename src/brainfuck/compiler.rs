use anyhow::{Result, bail};
use super::instructions::Instruction;

pub fn compile(code: &[u8]) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let mut jump_stack = Vec::new();
    let mut i: usize = 0;
    let len = code.len();

    while i < len {
        if i + 2 < len && code[i] == b'[' && (code[i + 1] == b'-' || code[i + 1] == b'+') && code[i + 2] == b']' {
            instructions.push(Instruction::ClearCell);
            i += 3;
            continue;
        }
        
        match code[i] {
            b'>' | b'<' => {
                let count = match code[i] {
                    b'>' => 1,
                    b'<' => -1,
                    _ => unreachable!(),
                };

                let mut total = count;
                while let Some(&next) = code.get(i + 1) {
                    if next == code[i] {
                        total += count;
                        i += 1;
                    } else {
                        break;
                    }
                }

                if total != 0 {
                    instructions.push(Instruction::Move(total));
                }
            },
            b'+' | b'-' => {
                let count = match code[i] {
                    b'+' => 1,
                    b'-' => -1,
                    _ => unreachable!(),
                };

                let mut total = count;
                while let Some(&next) = code.get(i + 1) {
                    if next == code[i] {
                        total += count;
                        i += 1;
                    } else {
                        break;
                    }
                }

                if total != 0 {
                    instructions.push(Instruction::Add(total));
                }
            },
            b'.' => {
                let mut count = 1;
                while let Some(&next) = code.get(i + 1) {
                    if next == b'.' {
                        count += 1;
                        i += 1;
                    } else {
                        break;
                    }
                }
                if count == 1 {
                    instructions.push(Instruction::Output);
                } else {
                    instructions.push(Instruction::OutputN(count));
                }
            },
            b',' => instructions.push(Instruction::Input),
            b'[' => {
                instructions.push(Instruction::JumpIfZero(0));
                jump_stack.push(instructions.len() - 1);
            },
            b']' => {
                if let Some(jump_index) = jump_stack.pop() {
                    instructions.push(Instruction::JumpIfNonZero(jump_index));
                    let jmp = instructions.len() - 1;
                    instructions[jump_index] = Instruction::JumpIfZero(jmp);
                } else {
                    bail!("Unmatched ']' at position {}", i);
                }
            },
            _ => {}
        }
        i += 1;
    }

    if jump_stack.is_empty() {
        Ok(instructions)
    } else {
        bail!("Unmatched '['")
    }
}