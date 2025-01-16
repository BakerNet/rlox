use std::fmt::Display;

use crate::location::SourceLocation;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BasicToken {
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KeywordToken {
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LiteralToken {
    String,
    Number,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}

impl Literal {
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Literal::False => false,
            Literal::Nil => false,
            _ => true,
        }
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Basic(BasicToken),
    Keyword(KeywordToken),
    Identifier,
    Literal(LiteralToken),
    EoF,
}

impl TokenType {
    pub fn from_string(s: &str) -> Option<TokenType> {
        match s {
            "and" => Some(TokenType::Keyword(KeywordToken::And)),
            "class" => Some(TokenType::Keyword(KeywordToken::Class)),
            "else" => Some(TokenType::Keyword(KeywordToken::Else)),
            "false" => Some(TokenType::Keyword(KeywordToken::False)),
            "fun" => Some(TokenType::Keyword(KeywordToken::Fun)),
            "for" => Some(TokenType::Keyword(KeywordToken::For)),
            "if" => Some(TokenType::Keyword(KeywordToken::If)),
            "nil" => Some(TokenType::Keyword(KeywordToken::Nil)),
            "or" => Some(TokenType::Keyword(KeywordToken::Or)),
            "print" => Some(TokenType::Keyword(KeywordToken::Print)),
            "return" => Some(TokenType::Keyword(KeywordToken::Return)),
            "super" => Some(TokenType::Keyword(KeywordToken::Super)),
            "this" => Some(TokenType::Keyword(KeywordToken::This)),
            "true" => Some(TokenType::Keyword(KeywordToken::True)),
            "var" => Some(TokenType::Keyword(KeywordToken::Var)),
            "while" => Some(TokenType::Keyword(KeywordToken::While)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenItem<'a> {
    pub ttype: TokenType,
    pub lexeme: &'a str,
    pub literal: Option<Literal>,
    pub location: SourceLocation,
}
