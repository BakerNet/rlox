use core::panic;

use crate::{
    Chunk, OpCode, Value,
    scan::{Precedence, Scanner, Token, TokenType},
};

pub(crate) struct Compiler;

impl Compiler {
    pub(crate) fn compile(source: &str, chunk: &mut Chunk) -> bool {
        let scanner = Scanner::new(source);
        let mut parser = Parser::new(scanner);
        parser.expression(chunk);
        parser.consume(TokenType::EoF, "Expected end of expression");
        chunk.write(OpCode::Return.into(), parser.previous.line);
        #[cfg(debug_assertions)]
        {
            if !parser.had_error {
                chunk.dissassemble("code");
            }
        }
        !parser.had_error
    }
}

struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    fn new(scanner: Scanner<'a>) -> Self {
        let mut parser = Self {
            scanner,
            current: Token::empty(),
            previous: Token::empty(),
            had_error: false,
            panic_mode: false,
        };
        // prime the pump
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        let new_current = loop {
            let try_token = self.scanner.scan_token();
            if try_token.ttype != TokenType::Error {
                break try_token;
            }
            self.error(try_token, "Unexpected token");
        };
        self.previous = self.current;
        self.current = new_current;
    }

    fn consume(&mut self, ttype: TokenType, message: &str) {
        if self.current.ttype != ttype {
            self.error(self.current, message);
        } else {
            self.advance();
        }
    }

    fn error(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}]", token.line);
        match token.ttype {
            TokenType::EoF => {
                eprint!(" at end");
            }
            TokenType::Error => {}
            _ => {
                eprint!(" af '{}'", token.lexeme)
            }
        }
        eprintln!(": {}", message);
        self.had_error = true;
    }

    fn expression(&mut self, chunk: &mut Chunk) {
        self.parse_precedence(Precedence::Assignment, chunk);
    }

    fn parse_precedence(&mut self, prec: Precedence, chunk: &mut Chunk) {
        self.advance();
        match self.previous.ttype {
            TokenType::LeftParen => {
                // grouping
                self.expression(chunk);
                self.consume(TokenType::RightParen, "Expected ')' after expression");
            }
            TokenType::Minus => {
                self.unary(chunk);
            }
            TokenType::Number => self.number(chunk),
            _ => {
                self.error(self.previous, "Expected expression");
                return;
            }
        }

        while prec <= self.current.ttype.precendence() {
            self.advance();
            match self.previous.ttype {
                TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                    self.binary(chunk);
                }
                _ => {}
            }
        }
    }

    fn number(&mut self, chunk: &mut Chunk) {
        let val = self
            .previous
            .lexeme
            .parse::<f64>()
            .expect("Should be able to parse float");
        chunk.write_constant(Value::Number(val), self.previous.line);
    }

    fn unary(&mut self, chunk: &mut Chunk) {
        let op = self.previous.ttype;
        self.parse_precedence(Precedence::Unary, chunk);
        let op_code = match op {
            TokenType::Minus => OpCode::Negate,
            _ => panic!("Unary called on unexpected TokenType {}", op),
        };
        chunk.write(op_code.into(), self.previous.line);
    }

    fn binary(&mut self, chunk: &mut Chunk) {
        let op = self.previous.ttype;
        self.parse_precedence(op.precendence().next(), chunk);
        let op_code = match op {
            TokenType::Minus => OpCode::Subtract,
            TokenType::Plus => OpCode::Add,
            TokenType::Star => OpCode::Multiply,
            TokenType::Slash => OpCode::Divide,
            _ => panic!("Binay called on unexpected TokenType {}", op),
        };
        chunk.write(op_code.into(), self.previous.line);
    }
}
