use crate::{location::SourceLocation, token::TokenItem};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("TODO error at {location}")]
    TODO { location: SourceLocation },
}

// For chapter 6, we will only parse equality expressions.
struct Parser {
    source: Vec<TokenItem>,
}

impl Parser {
    fn new(source: Vec<TokenItem>) -> Self {
        Self { source }
    }

    fn parse(&mut self) -> Result<(), Error> {
        // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
        // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
        // term           → factor ( ( "-" | "+" ) factor )* ;
        // factor         → unary ( ( "/" | "*" ) unary )* ;
        // unary          → ( "!" | "-" ) unary | primary ;
        // primary        → "true" | "false" | "nil"
        //                | NUMBER | STRING | "(" expression ")"
        todo!()
    }
}
