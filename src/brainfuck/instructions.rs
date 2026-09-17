#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    /// Moves the data pointer by the signed offset, wrapping at memory boundaries.
    Move(i16),
    /// Adds the value to the current cell using wrapping arithmetic.
    Add(u8),
    /// Adds the current cell times the multiplier to an offset cell, then clears it.
    MulAdd { offset: i16, multiplier: u8 },
    /// Writes the current cell to standard output the given number of times.
    OutputN(usize),
    /// Reads one byte into the current cell, using zero when no byte is available.
    Input,
    /// Jumps to the instruction index when the current cell is zero.
    JumpIfZero(usize),
    /// Jumps to the instruction index when the current cell is not zero.
    JumpIfNonZero(usize),
    /// Sets the current cell to zero.
    ClearCell,
    /// Moves left until the current cell is zero.
    ScanLeft,
    /// Moves right until the current cell is zero.
    ScanRight,
}
