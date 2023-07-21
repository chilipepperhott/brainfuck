use std::{
    fs::File,
    io::{self, Read},
};

use structopt::StructOpt;

use crate::{interpreter::Interpreter, parser::Parser, tokens::Token};

mod interpreter;
mod op;
mod parser;
mod tokens;

#[derive(Debug, StructOpt)]
#[structopt(name = "brainfuck", about = "A Brainfuck Interpreter.")]
struct Opts {
    pub file: String,
}

fn main() {
    let opts = Opts::from_args();

    let mut file = File::open(opts.file).expect("Could not open file.");
    let mut program = String::new();
    file.read_to_string(&mut program)
        .expect("Could not read from file.");

    let tokens = Token::lex_from_iter(program.chars());
    let program = Parser::parse_full(tokens).unwrap();

    let mut interpreter = Interpreter::new(program);

    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    loop {
        if interpreter.step(&mut stdin, &mut stdout).unwrap() {
            break;
        }
    }
}
