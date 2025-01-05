#![allow(dead_code)]
use std::io::Write;
use thiserror::Error;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

mod ast;
mod interpreter;
mod location;
mod parser;
mod scanner;
mod token;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}Scanning failed, see errors above.", .0.iter().fold(String::new(), |acc, e| acc + &e.to_string() + "\n"))]
    Scanner(Vec<crate::scanner::Error>),

    #[error("{}Parsing failed, see errors above.", .0.iter().fold(String::new(), |acc, e| acc + &e.to_string() + "\n"))]
    Parser(Vec<crate::parser::Error>),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub struct Lox {}

impl Lox {
    pub fn run() {
        todo!()
    }

    pub fn run_prompt() -> Result<(), Error> {
        loop {
            print!(">");
            std::io::stdout().flush()?;
            let mut line = String::new();
            if std::io::stdin().read_line(&mut line)? > 0 {
                let tokens = match Scanner::new().scan(&line).map_err(Error::Scanner) {
                    Ok(tokens) => tokens,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                let ast = match Parser::new().parse(tokens).map_err(Error::Parser) {
                    Ok(ast) => ast,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                let res = match Interpreter::new().interpret(&ast) {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                println!("{}", res);
            } else {
                break;
            }
        }
        Ok(())
    }
}
