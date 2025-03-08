use core::panic;

use crate::{
    Chunk, OpCode, Value,
    scan::{Precedence, Scanner, Token, TokenType},
};

pub(crate) struct Compiler;

impl Compiler {
    pub(crate) fn compile<'a>(source: &'a str, chunk: &mut Chunk<'a>) -> bool {
        let scanner = Scanner::new(source);
        let mut parser = Parser::new(scanner);
        while !parser.match_token(TokenType::EoF) {
            parser.declaration(chunk);
        }
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

    fn match_token(&mut self, ttype: TokenType) -> bool {
        if self.current.ttype != ttype {
            false
        } else {
            self.advance();
            true
        }
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
                eprint!(" after '{}'", token.lexeme)
            }
        }
        eprintln!(": {}", message);
        self.had_error = true;
    }

    fn declaration(&mut self, chunk: &mut Chunk<'a>) {
        if self.match_token(TokenType::Var) {
            self.var_declaration(chunk);
        } else {
            self.statement(chunk);
        }
        if self.panic_mode {
            self.synchronize();
        }
    }

    fn statement(&mut self, chunk: &mut Chunk<'a>) {
        if self.match_token(TokenType::Print) {
            self.print_statement(chunk);
        } else {
            // expression statement
            self.expression(chunk);
            self.consume(TokenType::Semicolon, "Expect ';' after expression");
            chunk.write(OpCode::Pop.into(), self.previous.line);
        }
    }

    fn expression<'b: 'a>(&mut self, chunk: &mut Chunk<'a>) {
        self.parse_precedence(Precedence::Assignment, chunk);
    }

    fn parse_precedence(&mut self, prec: Precedence, chunk: &mut Chunk<'a>) {
        self.advance();

        let can_assign = prec.can_assign();
        match self.previous.ttype {
            TokenType::LeftParen => {
                // grouping
                self.expression(chunk);
                self.consume(TokenType::RightParen, "Expected ')' after expression");
            }
            TokenType::Minus | TokenType::Bang => {
                self.unary(chunk);
            }
            TokenType::Number => self.number(chunk),
            TokenType::String => self.string(chunk),
            TokenType::Nil => {
                chunk.write(OpCode::Nil.into(), self.previous.line);
            }
            TokenType::True => {
                chunk.write(OpCode::True.into(), self.previous.line);
            }
            TokenType::False => {
                chunk.write(OpCode::False.into(), self.previous.line);
            }
            TokenType::Identifier => {
                let arg = chunk.add_constant(Value::ConstString(self.previous.lexeme));
                if can_assign && self.match_token(TokenType::Equal) {
                    self.expression(chunk);
                    chunk.write(OpCode::SetGlobal.into(), self.previous.line);
                    chunk.write(arg as u8, self.previous.line);
                } else {
                    chunk.write(OpCode::GetGlobal.into(), self.previous.line);
                    chunk.write(arg as u8, self.previous.line);
                }
            }
            _ => {
                self.error(self.previous, "Expected expression");
                return;
            }
        }

        while prec <= self.current.ttype.precendence() {
            self.advance();
            match self.previous.ttype {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Star
                | TokenType::Slash
                | TokenType::BangEqual
                | TokenType::EqualEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    self.binary(chunk);
                }
                _ => {}
            }
        }

        if can_assign && self.match_token(TokenType::Equal) {
            self.error(self.previous, "Invalid assign target");
        }
    }

    fn number<'b: 'a>(&mut self, chunk: &mut Chunk<'a>) {
        let val = self
            .previous
            .lexeme
            .parse::<f64>()
            .expect("Should be able to parse float");
        chunk.write_constant(Value::Number(val), self.previous.line);
    }

    fn string<'b: 'a>(&mut self, chunk: &mut Chunk<'a>) {
        let lexeme = self.previous.lexeme;
        let str = &lexeme[1..lexeme.len() - 1]; // remove quotes
        chunk.write_constant(Value::ConstString(str), self.previous.line);
    }

    fn unary(&mut self, chunk: &mut Chunk<'a>) {
        let op = self.previous.ttype;
        self.parse_precedence(Precedence::Unary, chunk);
        let op_code = match op {
            TokenType::Minus => OpCode::Negate,
            TokenType::Bang => OpCode::Not,
            _ => panic!("Unary called on unexpected TokenType {}", op),
        };
        chunk.write(op_code.into(), self.previous.line);
    }

    fn binary(&mut self, chunk: &mut Chunk<'a>) {
        let op = self.previous.ttype;
        self.parse_precedence(op.precendence().next(), chunk);
        let (op_code1, op_code2) = match op {
            TokenType::Minus => (OpCode::Subtract, None),
            TokenType::Plus => (OpCode::Add, None),
            TokenType::Star => (OpCode::Multiply, None),
            TokenType::Slash => (OpCode::Divide, None),
            TokenType::BangEqual => (OpCode::Equal, Some(OpCode::Not)),
            TokenType::EqualEqual => (OpCode::Equal, None),
            TokenType::Greater => (OpCode::Greater, None),
            TokenType::GreaterEqual => (OpCode::Greater, Some(OpCode::Not)),
            TokenType::Less => (OpCode::Less, None),
            TokenType::LessEqual => (OpCode::Less, Some(OpCode::Not)),
            _ => panic!("Binay called on unexpected TokenType {}", op),
        };
        chunk.write(op_code1.into(), self.previous.line);
        if let Some(oc) = op_code2 {
            chunk.write(oc.into(), self.previous.line);
        }
    }

    fn print_statement(&mut self, chunk: &mut Chunk<'a>) {
        self.expression(chunk);
        self.consume(TokenType::Semicolon, "Expect ; after value.");
        chunk.write(OpCode::Print.into(), self.previous.line);
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;

        while !matches!(self.current.ttype, TokenType::EoF) {
            if matches!(self.previous.ttype, TokenType::Semicolon) {
                return;
            };
            match self.current.ttype {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn var_declaration(&mut self, chunk: &mut Chunk<'a>) {
        self.consume(TokenType::Identifier, "Expected variable name.");
        let name = self.previous.lexeme;
        if self.match_token(TokenType::Equal) {
            self.expression(chunk);
        } else {
            chunk.write(OpCode::Nil.into(), self.previous.line);
        }
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        );
        let global = chunk.add_constant(Value::ConstString(name));
        chunk.write(OpCode::DefineGlobal.into(), self.previous.line);
        chunk.write(global as u8, self.previous.line);
    }
}
