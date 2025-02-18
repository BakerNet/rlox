use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{ast::Stmt, environment::Environment, location::SourceLocation};

#[derive(Debug, Clone)]
pub enum Literal {
    Function {
        params: Vec<&'static str>,
        body: Rc<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
    String(Rc<String>),
    Number(f64),
    True,
    False,
    Nil,
}

impl Literal {
    pub(crate) fn is_truthy(&self) -> bool {
        !matches!(self, Literal::Nil | Literal::False)
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        if b { Literal::True } else { Literal::False }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Function { .. } => write!(f, "function"),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Function { .. }, _) => false,
            (_, Self::Function { .. }) => false,
            (Literal::String(a), Literal::String(b)) => a == b,
            (Literal::Number(a), Literal::Number(b)) => a == b,
            (Literal::True, Literal::True) => true,
            (Literal::False, Literal::False) => true,
            (Literal::Nil, Literal::Nil) => true,
            (_, _) => false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // Basic
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEq,
    Equal,
    EqualEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    // Keyword
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Literal
    String,
    Number,
    // Other
    Identifier,
    EoF,
}

impl TokenType {
    pub fn from_string(s: &str) -> Option<TokenType> {
        match s {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenItem {
    pub ttype: TokenType,
    pub lexeme: &'static str,
    pub literal: Option<Literal>,
    pub location: SourceLocation,
}
