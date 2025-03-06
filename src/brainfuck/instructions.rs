#[derive(Debug)]
pub enum Instruction {
    Move(isize),
    Add(isize),
    Output,
    Input,
    JumpIfZero(usize),
    JumpIfNonZero(usize),
}