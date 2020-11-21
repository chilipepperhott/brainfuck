mod interpreter;

use std::{
    fs::File,
    io::{Read, Write},
    time::Duration,
};

use crossterm::event::{poll, read, Event, KeyCode};
use interpreter::BrainfuckInterpreter;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "brainfuck", about = "A Brainfuck Interpreter.")]
struct Opts {
    pub file: String,
    /// Whether to read from stdin. Off by default for performance reasons.
    #[structopt(short, long)]
    pub stdin: bool,
    #[structopt(short, long)]
    /// Initial input. Use this if you don't want to enable stdin but still want some input.
    pub input: Option<String>,
}

fn main() {
    let opts = Opts::from_args();

    let mut file = File::open(opts.file).expect("Could not open file.");
    let mut program = String::new();
    file.read_to_string(&mut program)
        .expect("Could not read from file.");

    let mut interpreter = BrainfuckInterpreter::new(&program).unwrap();

    if let Some(input) = opts.input {
        interpreter.write_input_string(&input);
    }

    while match interpreter.step() {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    } {
        if let Some(c) = interpreter.read_output() {
            print!("{}", c as char);
            std::io::stdout().flush().unwrap();
        }

        if opts.stdin {
            if let Ok(true) = poll(Duration::ZERO) {
                if let Event::Key(key_event) = read().unwrap() {
                    if let KeyCode::Char(c) = key_event.code {
                        interpreter.write_input(c as u8);
                    }
                }
            }
        }
    }
}
