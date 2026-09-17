use super::instructions::Instruction;
use anyhow::{bail, Result};

pub(super) fn compile(code: &[u8]) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::with_capacity(code.len());
    let mut open_loop = None;
    let mut saw_instruction = false;
    let mut i = 0;

    while i < code.len() {
        match code[i] {
            b'>' | b'<' => {
                saw_instruction = true;
                let (delta, skip) = fold_delta(code, i, b'>', b'<');
                let delta = delta as i16;
                if delta != 0 {
                    instructions.push(Instruction::Move(delta));
                }
                i += skip;
            }
            b'+' | b'-' => {
                saw_instruction = true;
                let (delta, skip) = fold_delta(code, i, b'+', b'-');
                let delta = delta as u8;
                if delta != 0 {
                    instructions.push(Instruction::Add(delta));
                }
                i += skip;
            }
            b'.' => {
                saw_instruction = true;
                let count = count_repeats(code, i, b'.');
                instructions.push(Instruction::OutputN(count));
                i += count;
            }
            b',' => {
                saw_instruction = true;
                instructions.push(Instruction::Input);
                i += 1;
            }
            b'[' => {
                saw_instruction = true;
                if let Some((instruction, skip)) = try_optimize_loop(code, i) {
                    instructions.push(instruction);
                    i += skip;
                    continue;
                }

                let start_idx = instructions.len();
                let previous_start_idx = open_loop.unwrap_or(start_idx);
                instructions.push(Instruction::JumpIfZero(previous_start_idx));
                open_loop = Some(start_idx);
                i += 1;
            }
            b']' => {
                saw_instruction = true;
                if let Some(start_idx) = open_loop {
                    let Instruction::JumpIfZero(previous_start_idx) = instructions[start_idx]
                    else {
                        unreachable!()
                    };
                    open_loop = (previous_start_idx != start_idx).then_some(previous_start_idx);

                    let end_idx = instructions.len();
                    instructions.push(Instruction::JumpIfNonZero(start_idx));
                    instructions[start_idx] = Instruction::JumpIfZero(end_idx);
                } else {
                    bail!("Unmatched ']' at position {}", i);
                }
                i += 1;
            }
            _ => i += 1,
        }
    }

    if !saw_instruction {
        bail!("Source does not contain Brainfuck instructions.")
    }
    if open_loop.is_some() {
        bail!("Unmatched '['")
    }

    Ok(instructions)
}

fn fold_delta(code: &[u8], start: usize, positive: u8, negative: u8) -> (isize, usize) {
    let mut delta = 0;
    let mut idx = start;

    while idx < code.len() {
        match code[idx] {
            byte if byte == positive => delta += 1,
            byte if byte == negative => delta -= 1,
            _ => break,
        }
        idx += 1;
    }

    (delta, idx - start)
}

fn count_repeats(code: &[u8], start: usize, target: u8) -> usize {
    code[start..]
        .iter()
        .take_while(|&&byte| byte == target)
        .count()
}

fn try_optimize_loop(code: &[u8], i: usize) -> Option<(Instruction, usize)> {
    if code.get(i + 2) == Some(&b']') {
        return match code[i + 1] {
            b'-' | b'+' => Some((Instruction::ClearCell, 3)),
            b'>' => Some((Instruction::ScanRight, 3)),
            b'<' => Some((Instruction::ScanLeft, 3)),
            _ => None,
        };
    }

    try_optimize_mul_add(code, i)
}

fn try_optimize_mul_add(code: &[u8], i: usize) -> Option<(Instruction, usize)> {
    if code.get(i + 1) != Some(&b'-') {
        return None;
    }

    let mut cursor = i + 2;
    let (offset_delta, move_count) = fold_delta(code, cursor, b'>', b'<');
    cursor += move_count;

    let (multiplier, add_count) = fold_delta(code, cursor, b'+', b'-');
    cursor += add_count;

    let (return_offset, return_count) = fold_delta(code, cursor, b'>', b'<');
    cursor += return_count;

    let offset = offset_delta as i16;
    let multiplier = multiplier as u8;
    if move_count == 0
        || add_count == 0
        || return_count == 0
        || offset == 0
        || multiplier == 0
        || offset_delta + return_offset != 0
        || code.get(cursor) != Some(&b']')
    {
        return None;
    }

    Some((Instruction::MulAdd { offset, multiplier }, cursor - i + 1))
}