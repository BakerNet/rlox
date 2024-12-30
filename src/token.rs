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
    Identifier,
    String,
    Number,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Basic(BasicToken),
    KeywordToken(KeywordToken),
    Literal(LiteralToken),
    EOF,
}

impl TokenType {
    pub fn from_string(s: &str) -> Option<TokenType> {
        match s {
            "and" => Some(TokenType::KeywordToken(KeywordToken::And)),
            "class" => Some(TokenType::KeywordToken(KeywordToken::Class)),
            "else" => Some(TokenType::KeywordToken(KeywordToken::Else)),
            "false" => Some(TokenType::KeywordToken(KeywordToken::False)),
            "fun" => Some(TokenType::KeywordToken(KeywordToken::Fun)),
            "for" => Some(TokenType::KeywordToken(KeywordToken::For)),
            "if" => Some(TokenType::KeywordToken(KeywordToken::If)),
            "nil" => Some(TokenType::KeywordToken(KeywordToken::Nil)),
            "or" => Some(TokenType::KeywordToken(KeywordToken::Or)),
            "print" => Some(TokenType::KeywordToken(KeywordToken::Print)),
            "return" => Some(TokenType::KeywordToken(KeywordToken::Return)),
            "super" => Some(TokenType::KeywordToken(KeywordToken::Super)),
            "this" => Some(TokenType::KeywordToken(KeywordToken::This)),
            "true" => Some(TokenType::KeywordToken(KeywordToken::True)),
            "var" => Some(TokenType::KeywordToken(KeywordToken::Var)),
            "while" => Some(TokenType::KeywordToken(KeywordToken::While)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TokenItem {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub location: SourceLocation,
}

impl TokenItem {
    pub fn to_string(&self) -> String {
        format!("{:?} {:?} {:?}", self.ttype, self.lexeme, self.literal)
    }
}
