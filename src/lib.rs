use std::io::Write;
use thiserror::Error;

use scanner::Scanner;

mod scanner;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Scanning failed.")]
    Scanner(),

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
                let tokens = Scanner::new(&line).scan();
                dbg!(tokens);
            } else {
                break;
            }
        }
        Ok(())
    }
}
