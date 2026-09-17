use super::instructions::Instruction;
use anyhow::Result;
use std::io::{self, Read, Write};

const MEM_SIZE: usize = 65536;

pub trait MachineTrait {
    fn get_val(&self) -> u8;
    fn set_val(&mut self, val: u8);
    fn add_val(&mut self, delta: u8);
    fn move_ptr(&mut self, delta: i16);
    fn scan_left(&mut self);
    fn scan_right(&mut self);
}

struct StaticMachine {
    memory: [u8; MEM_SIZE],
    ptr: u16,
}

impl StaticMachine {
    fn new() -> Self {
        Self {
            memory: [0u8; MEM_SIZE],
            ptr: 0,
        }
    }
}

impl MachineTrait for StaticMachine {
    #[inline(always)]
    fn get_val(&self) -> u8 {
        // SAFETY: self.ptr is u16 (0..65535), which is always within
        // the bounds of [u8; 65536].
        unsafe { *self.memory.get_unchecked(self.ptr as usize) }
    }

    #[inline(always)]
    fn set_val(&mut self, val: u8) {
        // SAFETY: Indexing is safe because u16 cannot exceed MEM_SIZE - 1.
        unsafe {
            *self.memory.get_unchecked_mut(self.ptr as usize) = val;
        }
    }

    #[inline(always)]
    fn add_val(&mut self, delta: u8) {
        let current = self.get_val();
        self.set_val(current.wrapping_add(delta));
    }

    #[inline(always)]
    fn move_ptr(&mut self, delta: i16) {
        self.ptr = self.ptr.wrapping_add(delta as u16);
    }

    #[inline(always)]
    fn scan_left(&mut self) {
        // SAFETY: Pointer is guaranteed to be within 0..65535.
        unsafe {
            while *self.memory.get_unchecked(self.ptr as usize) != 0 {
                self.ptr = self.ptr.wrapping_sub(1);
            }
        }
    }

    #[inline(always)]
    fn scan_right(&mut self) {
        // SAFETY: Pointer is guaranteed to be within 0..65535.
        unsafe {
            while *self.memory.get_unchecked(self.ptr as usize) != 0 {
                self.ptr = self.ptr.wrapping_add(1);
            }
        }
    }
}

pub(super) fn run(instructions: &[Instruction]) -> Result<()> {
    let mut machine = StaticMachine::new();
    execute(&mut machine, instructions)
}

fn execute<M: MachineTrait>(machine: &mut M, instructions: &[Instruction]) -> Result<()> {
    let mut ip: usize = 0;
    let len = instructions.len();
    let inst_ptr = instructions.as_ptr();

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut stdin = io::stdin();
    let mut input_buf = [0u8; 1];

    while ip < len {
        // SAFETY: ip is checked against len, so offset is valid.
        let instr = unsafe { &*inst_ptr.add(ip) };

        match instr {
            Instruction::Move(delta) => machine.move_ptr(*delta),
            Instruction::Add(val) => machine.add_val(*val),
            Instruction::MulAdd { offset, multiplier } => {
                let value = machine.get_val();
                machine.move_ptr(*offset);
                machine.add_val(value.wrapping_mul(*multiplier));
                machine.move_ptr((*offset).wrapping_neg());
                machine.set_val(0);
            }
            Instruction::OutputN(n) => {
                io::copy(&mut io::repeat(machine.get_val()).take(*n as u64), &mut out)?;
            }
            Instruction::Input => {
                input_buf[0] = 0;
                let _ = stdin.read_exact(&mut input_buf);
                machine.set_val(input_buf[0]);
            }
            Instruction::JumpIfZero(target) => {
                if machine.get_val() == 0 {
                    ip = *target;
                    continue;
                }
            }
            Instruction::JumpIfNonZero(target) => {
                if machine.get_val() != 0 {
                    ip = *target;
                    continue;
                }
            }
            Instruction::ClearCell => machine.set_val(0),
            Instruction::ScanLeft => machine.scan_left(),
            Instruction::ScanRight => machine.scan_right(),
        }
        ip += 1;
    }

    out.flush()?;
    Ok(())
}
