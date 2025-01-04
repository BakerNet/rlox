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
    True,
    False,
    Nil,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Basic(BasicToken),
    Keyword(KeywordToken),
    Literal(LiteralToken),
    EOF,
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
