use std::io::{Read, Write};

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
    pub fn step(
        &mut self,
        input: &mut impl Read,
        output: &mut impl Write,
    ) -> std::io::Result<bool> {
        let cur_op = self.program[self.pc];
        let cur_m = &mut self.memory[self.mp];

        match cur_op {
            Op::Left(n) => self.mp -= n as usize,
            Op::Right(n) => self.mp += n as usize,
            Op::Inc(n) => *cur_m = (*cur_m).overflowing_add(n).0,
            Op::Dec(n) => *cur_m = (*cur_m).overflowing_sub(n).0,
            Op::Put => output.write_all(&[*cur_m])?,
            Op::Get => {
                let mut buf = [0; 1];
                input.read_exact(&mut buf)?;
                *cur_m = buf[0];
            }
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
