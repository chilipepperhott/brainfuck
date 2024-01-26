use crate::{op::Op, tokens::Token};

pub type Program = Vec<Op>;

#[derive(Debug)]
pub struct Parser {
    parsed: Program,
    /// A stack of the indexes of [`Op::Open`] in the program.
    /// Elements are removed when a matching [`Op::Close`] is found.
    jump_stack: Vec<usize>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            /// TODO: Estimate capacity?
            parsed: Vec::new(),
            jump_stack: Vec::new(),
        }
    }

    pub fn parse_token(&mut self, token: Token) -> Result<(), Error> {
        macro_rules! push_or_inc {
            ($action:ident) => {
                if let Some(Op::$action(n)) = self.parsed.last_mut() {
                    if (*n < 255) {
                        *n += 1
                    } else {
                        self.parsed.push(Op::$action(1))
                    }
                } else {
                    self.parsed.push(Op::$action(1))
                }
            };
        }

        match token {
            Token::Left => push_or_inc!(Left),
            Token::Right => push_or_inc!(Right),
            Token::Inc => push_or_inc!(Inc),
            Token::Dec => push_or_inc!(Dec),
            Token::Put => self.parsed.push(Op::Put),
            Token::Get => self.parsed.push(Op::Get),
            // Set `jump_to` to 0. This will be overwritten when the close tag is processed.
            Token::Open => {
                self.jump_stack.push(self.parsed.len());
                self.parsed.push(Op::Open { jump_to: 0 });
            }
            Token::Close => {
                let open_index = self.jump_stack.pop().ok_or(Error::UnmatchedCloseTag)?;

                let op_index = self.parsed.len();

                let Op::Open { jump_to } = &mut self.parsed[open_index] else{
                    return Err(Error::ExpectedOpInProgram { expected: Token::Open, found: self.parsed[open_index].into() })
                };

                *jump_to = op_index;

                self.parsed.push(Op::Close {
                    jump_to: open_index,
                });
            }
        }

        Ok(())
    }

    pub fn parse_full<'a>(tokens: impl Iterator<Item = Token> + 'a) -> Result<Program, Error> {
        let mut parser = Self::new();

        for token in tokens {
            parser.parse_token(token)?;
        }

        Ok(parser.parsed)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An unmatched close tag was provided.")]
    UnmatchedCloseTag,
    #[error("The parser internal state was invalid. Expected {expected:?}, found {found:?}.")]
    ExpectedOpInProgram { expected: Token, found: Token },
}
