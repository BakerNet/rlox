#![allow(dead_code)]
mod chunk;
mod compiler;
mod scan;
mod value;
mod vm;

pub use chunk::{Chunk, OpCode};
pub use value::Value;
pub use vm::VM;

use std::io::Write;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Compiler Error")]
    Compiler,

    #[error("Runtime Error")]
    Runtime,

    #[error("IO Error")]
    Io,
}

pub struct Lox();

impl Lox {
    pub fn run(file: String) -> Result<(), Error> {
        VM::new().interpret(&file)
    }

    pub fn run_prompt() -> Result<(), Error> {
        let mut vm = VM::new();
        loop {
            print!(">");
            std::io::stdout().flush().map_err(|_| Error::Io)?;
            let mut line = String::new();
            if std::io::stdin()
                .read_line(&mut line)
                .map_err(|_| Error::Io)?
                > 0
            {
                // lines need to be leaked because global variables persist
                let line = line.leak();
                let res = vm.interpret(line);
                if let Err(e) = res {
                    println!("Error: {}", e);
                }
            }
        }
    }
}
