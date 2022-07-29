use crate::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Move(isize),
    Increment(isize),
    Output,
    Input,
    JumpOpen(usize),
    JumpClose(usize),
}

#[derive(Debug, Clone)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn from_text(text: &str) -> Result<Self, Error> {
        let program_text = text.as_bytes();

        let mut program = Vec::new();
        let mut scan_pointer = 0;
        let mut jump_opens = Vec::new();

        /// Scans forwards counting repetitions of a character
        fn count_repetitions(
            scan_pointer: &mut usize,
            program_text: &[u8],
            character: u8,
        ) -> isize {
            let mut count = 1;
            while {
                if let Some(v) = program_text.get(*scan_pointer + 1) {
                    program_text[*scan_pointer + 1] == character
                } else {
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
                b'>' => program.push(Instruction::Move(count_repetitions(
                    &mut scan_pointer,
                    program_text,
                    b'>',
                ))),
                b'<' => program.push(Instruction::Move(-count_repetitions(
                    &mut scan_pointer,
                    program_text,
                    b'<',
                ))),
                b'+' => program.push(Instruction::Increment(count_repetitions(
                    &mut scan_pointer,
                    program_text,
                    b'+',
                ))),
                b'-' => program.push(Instruction::Increment(-count_repetitions(
                    &mut scan_pointer,
                    program_text,
                    b'-',
                ))),
                b'.' => program.push(Instruction::Output),
                b',' => program.push(Instruction::Input),
                b'[' => {
                    jump_opens.push(program.len());
                    program.push(Instruction::JumpOpen(0))
                }
                b']' => match jump_opens.pop() {
                    Some(location) => {
                        program[location] = Instruction::JumpOpen(program.len());
                        program.push(Instruction::JumpClose(location));
                    }
                    None => return Err(Error::UnmatchedBracket(scan_pointer)),
                },
                _ => (),
            }

            scan_pointer += 1;
        }

        if let Some(unmatched_bracket) = jump_opens.pop() {
            return Err(Error::UnmatchedBracket(unmatched_bracket));
        }

        Ok(Self {
            instructions: program,
        })
    }

    pub fn get_instruction(&self, index: usize) -> Option<Instruction> {
        self.instructions.get(index).cloned()
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }
}
