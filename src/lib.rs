#![allow(dead_code)]
use std::fmt::Debug;
use std::io::Write;
use thiserror::Error;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

mod ast;
mod environment;
mod interpreter;
mod location;
mod parser;
mod scanner;
mod token;

#[derive(Error)]
pub enum Error {
    #[error("{}Scanning failed, see errors above.", .0.iter().fold(String::new(), |acc, e| acc + &e.to_string() + "\n"))]
    Scanner(Vec<crate::scanner::Error>),

    #[error("{}Parsing failed, see errors above.", .0.iter().fold(String::new(), |acc, e| acc + &e.to_string() + "\n"))]
    Parser(Vec<crate::parser::Error>),

    #[error(transparent)]
    Runtime(#[from] interpreter::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Lox {}

impl Lox {
    pub fn run(file: String) -> Result<(), Error> {
        let tokens = Scanner::new().scan(&file).map_err(Error::Scanner)?;
        let ast = Parser::new().parse(tokens).map_err(Error::Parser)?;
        let res = Interpreter::new().interpret(ast).map_err(Error::Runtime)?;
        if let Some(res) = res {
            println!("{}", res);
        }
        Ok(())
    }

    pub fn run_prompt() -> Result<(), Error> {
        let interpreter = Interpreter::new();
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
                let res = match interpreter.interpret(ast) {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                if let Some(res) = res {
                    println!("{}", res);
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}
