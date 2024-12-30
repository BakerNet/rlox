use std::io::Write;
use thiserror::Error;

use scanner::Scanner;

mod location;
mod scanner;
mod token;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}Scanning failed, see errors above.", .0.iter().map(|e| format!("{}\n", e)).collect::<String>())]
    Scanner(Vec<crate::scanner::Error>),

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
                let tokens = match Scanner::new(&line).scan().map_err(Error::Scanner) {
                    Ok(tokens) => tokens,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                dbg!(tokens);
            } else {
                break;
            }
        }
        Ok(())
    }
}
