use std::io::Write;

use crate::{op::Op, parser::Program};

#[derive(Debug)]
pub struct Interpreter {
    pc: usize,
    program: Program,
    memory: Vec<u8>,
    mp: usize,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Self {
            pc: 0,
            program,
            memory: vec![0; 30_000],
            mp: 0,
        }
    }

    /// Bool returns whether the program is now complete. True = complete.
    /// Result returns any IO error that occurs.
    pub fn step(&mut self, out: &mut impl Write) -> std::io::Result<bool> {
        let cur_op = self.program[self.pc];
        let cur_m = &mut self.memory[self.mp];

        match cur_op {
            Op::Left => self.mp -= 1,
            Op::Right => self.mp += 1,
            Op::Inc => *cur_m = (*cur_m).overflowing_add(1).0,
            Op::Dec => *cur_m = (*cur_m).overflowing_sub(1).0,
            Op::Put => out.write_all(&[*cur_m])?,
            Op::Get => todo!(),
            Op::Open { jump_to } => {
                if *cur_m == 0 {
                    self.pc = jump_to;
                }
            }
            Op::Close { jump_to } => {
                if *cur_m != 0 {
                    self.pc = jump_to
                }
            }
        }

        self.pc += 1;

        Ok(self.pc >= self.program.len() - 1)
    }
}
