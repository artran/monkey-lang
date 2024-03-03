use std::io;
use std::io::Write;

use crate::{lexer::Lexer, token::Token};

pub(crate) struct Repl {
    prompt: String,
}

impl Repl {
    pub(crate) fn new() -> Repl {
        Repl {
            prompt: ">> ".to_string(),
        }
    }

    pub(crate) fn run(&mut self) {
        loop {
            print!("{}", self.prompt);
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.len() == 0 || input.trim() == "exit" {
                println!("Goodbye!");
                break;
            }

            let mut lexer = Lexer::new(input);
            loop {
                let token = lexer.next_token();
                if token == Token::EOF {
                    break;
                }
                println!("{:?}", token);
            }
        }
    }
}
