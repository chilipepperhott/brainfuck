use crate::error::Error;
use std::convert::TryInto;
use std::iter;
use std::{collections::VecDeque, fmt::Display};

use crate::program::{Instruction, Program};

#[derive(Debug)]
pub struct Interpreter {
    output: VecDeque<u8>,
    input: VecDeque<u8>,
    memory: Vec<u8>,
    memory_pointer: usize,
    program: Program,
    program_pointer: usize,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Interpreter {
            output: VecDeque::new(),
            input: VecDeque::new(),
            memory: vec![0; 30_000],
            memory_pointer: 0,
            program,
            program_pointer: 0,
        }
    }

    /// Steps the program forward once.
    /// If there is nothing in the input queue and it needs input, nothing happens.
    /// Returns false if the program has ended, otherwise it returns true
    pub fn step(&mut self) -> Result<bool, Error> {
        let instruction = match self.program.get_instruction(self.program_pointer){
            Some(instruction) => instruction,
            None => return Ok(false),
        };

        match instruction {
            Instruction::Move(n) => {
                if n >= 0{
                    self.memory_pointer += n as usize;
                }else{
                    self.memory_pointer -= -n as usize;
                }

                if self.memory_pointer >= self.memory.len(){
                    // Resize in larger chunks to avoid doing it a lot.
                    self.memory.resize(self.memory_pointer + 30_000, 0);
                }
            }
            Instruction::Increment(n) => {
                let cell_value =  self.memory[self.memory_pointer];

                self.memory[self.memory_pointer] = if n > 0{
                    cell_value.overflowing_add((n % 256) as u8).0
                }else{
                    cell_value.overflowing_sub((-n % 256) as u8).0
                };
            }
            Instruction::Output => {
                self.output.push_back(self.memory[self.memory_pointer]);
            }
            Instruction::Input => {
                self.memory[self.memory_pointer] = match self.input.pop_front() {
                    Some(value) => value,
                    None => return Ok(true),
                }
            }
            Instruction::JumpOpen(location) => {
                if self.memory[self.memory_pointer] == 0 {
                    self.program_pointer = location;
                }
            }
            Instruction::JumpClose(location) => {
                if self.memory[self.memory_pointer] != 0 {
                    self.program_pointer = location;
                }
            }
        }

        self.program_pointer += 1;
        Ok(self.program_pointer <= self.program.len())
    }

    /// Pops from the output queue
    pub fn read_output(&mut self) -> Option<u8> {
        self.output.pop_front()
    }

    /// Writes to the input queue
    pub fn write_input(&mut self, value: u8) {
        self.input.push_back(value);
    }

    /// Writes a string to the input queue
    pub fn write_input_string(&mut self, value: &str) {
        for item in value.chars() {
            self.input.push_back(item as u8);
        }
    }
}
