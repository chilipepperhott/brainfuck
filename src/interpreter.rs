use std::{collections::VecDeque, error::Error, fmt::Display};

#[derive(Debug)]
pub enum Command {
    MoveRight(usize),
    MoveLeft(usize),
    Increment(usize),
    Decrement(usize),
    Output,
    Input,
    JumpOpen(usize),
    JumpClose(usize),
}

#[derive(Debug)]
pub enum BrainfuckError {
    UnmatchedBracket(usize),
    OutOfBounds,
}

impl Error for BrainfuckError {}

impl Display for BrainfuckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnmatchedBracket(n) => write!(
                f,
                "There is an unmatched bracket at program position: {}",
                n
            ),
            Self::OutOfBounds => write!(f, "The program went to far to the left"),
        }
    }
}

#[derive(Debug)]
pub struct BrainfuckInterpreter {
    output: VecDeque<u8>,
    input: VecDeque<u8>,
    memory: Vec<u8>,
    memory_pointer: usize,
    program: Vec<Command>,
    program_pointer: usize,
}

impl BrainfuckInterpreter {
    pub fn new(program_text: &str) -> Result<Self, BrainfuckError> {
        use Command::*;
        let program_text = program_text.as_bytes();

        let mut program = Vec::new();
        let mut scan_pointer = 0;
        let mut jump_opens = Vec::new();

        /// Scans forwards counting repetitions of a character
        fn count_repetitions(
            scan_pointer: &mut usize,
            program_text: &[u8],
            character: u8,
        ) -> usize {
            let mut count = 1;
            while {
                if let Some(v) = program_text.get(*scan_pointer + 1) {
                    program_text[*scan_pointer + 1] == character
                }else{
                    false
                }
                
            } {
                *scan_pointer += 1;
                count += 1;
            }
            count
        }

        while scan_pointer < program_text.len() {
            match program_text[scan_pointer] {
                b'>' => program.push(MoveRight(count_repetitions(
                    &mut scan_pointer,
                    program_text,
                    b'>',
                ))),
                b'<' => program.push(MoveLeft(count_repetitions(
                    &mut scan_pointer,
                    program_text,
                    b'<',
                ))),
                b'+' => program.push(Increment(count_repetitions(
                    &mut scan_pointer,
                    program_text,
                    b'+',
                ))),
                b'-' => program.push(Decrement(count_repetitions(
                    &mut scan_pointer,
                    program_text,
                    b'-',
                ))),
                b'.' => program.push(Output),
                b',' => program.push(Input),
                b'[' => {
                    jump_opens.push(program.len());
                    program.push(JumpOpen(0))
                }
                b']' => match jump_opens.pop() {
                    Some(location) => {
                        program[location] = JumpOpen(program.len());
                        program.push(JumpClose(location));
                    }
                    None => return Err(BrainfuckError::UnmatchedBracket(scan_pointer)),
                },
                _ => (),
            }

            scan_pointer += 1;
        }

        if let Some(unmatched_bracket) = jump_opens.pop() {
            return Err(BrainfuckError::UnmatchedBracket(unmatched_bracket));
        }

        Ok(BrainfuckInterpreter {
            output: VecDeque::new(),
            input: VecDeque::new(),
            memory: vec![0; 30_000],
            memory_pointer: 0,
            program,
            program_pointer: 0,
        })
    }

    /// Steps the program forward once.
    /// If there is nothing in the input queue and it needs input, nothing happens.
    /// Returns false if the program has ended, otherwise it returns true
    pub fn step(&mut self) -> Result<bool, BrainfuckError> {
        if self.program_pointer >= self.program.len() {
            return Ok(false);
        }

        /// Checks if the operation being executed has required memory
        fn ensure_memory_exists(memory: &mut Vec<u8>, location: usize) {
            while memory.len() <= location {
                memory.push(0);
            }
        }

        match self.program[self.program_pointer] {
            Command::MoveRight(n) => {
                self.memory_pointer += n;
                ensure_memory_exists(&mut self.memory, self.memory_pointer);
            }
            Command::MoveLeft(n) => {
                if self.memory_pointer < n {
                    return Err(BrainfuckError::OutOfBounds);
                }

                self.memory_pointer -= n;
            }
            Command::Increment(n) => {
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer]
                    .overflowing_add((n % 256) as u8)
                    .0;
            }
            Command::Decrement(n) => {
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer]
                    .overflowing_sub((n % 256) as u8)
                    .0;
            }
            Command::Output => {
                self.output.push_back(self.memory[self.memory_pointer]);
            }
            Command::Input => {
                self.memory[self.memory_pointer] = match self.input.pop_front() {
                    Some(value) => value,
                    None => return Ok(true),
                }
            }
            Command::JumpOpen(location) => {
                if self.memory[self.memory_pointer] == 0 {
                    self.program_pointer = location;
                }
            }
            Command::JumpClose(location) => {
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
