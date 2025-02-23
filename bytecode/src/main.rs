use bytecode::{Error, Lox};
use std::fs::read_to_string;

pub fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    #[allow(clippy::comparison_chain)]
    if args.len() > 2 {
        println!("Usage: {} [script]", args[0]);
        std::process::exit(64);
    } else if args.len() == 2 {
        let contents = read_to_string(&args[1]).map_err(|_| Error::Io)?;
        // because lexemes are stored as &static str to reduce allocations, leak the contents
        Lox::run(contents)
    } else {
        Lox::run_prompt()
    }
}
