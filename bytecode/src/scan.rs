use std::fmt::Display;

pub enum TokenType {
    // Single-character tokens.
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
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
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

    Error,
    EoF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Star => write!(f, "Star"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::BangEqual => write!(f, "Bangequal"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::String => write!(f, "String"),
            TokenType::Number => write!(f, "Number"),
            TokenType::And => write!(f, "And"),
            TokenType::Class => write!(f, "Class"),
            TokenType::Else => write!(f, "Else"),
            TokenType::False => write!(f, "False"),
            TokenType::For => write!(f, "For"),
            TokenType::Fun => write!(f, "Fun"),
            TokenType::If => write!(f, "If"),
            TokenType::Nil => write!(f, "Nil"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Print => write!(f, "Print"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Super => write!(f, "Super"),
            TokenType::This => write!(f, "This"),
            TokenType::True => write!(f, "True"),
            TokenType::Var => write!(f, "Var"),
            TokenType::While => write!(f, "While"),
            TokenType::Error => write!(f, "Error"),
            TokenType::EoF => write!(f, "Eof"),
        }
    }
}

pub struct Token<'a> {
    pub(crate) ttype: TokenType,
    pub(crate) lexeme: &'a str,
    pub(crate) line: usize,
}

macro_rules! token {
    ($self:ident, $ttype:expr) => {
        Token {
            ttype: $ttype,
            lexeme: str::from_utf8(&$self.source[$self.start..$self.current]).unwrap(),
            line: $self.line,
        }
    };
}

macro_rules! error_token {
    ($self:ident, $message:expr) => {
        Token {
            ttype: TokenType::Error,
            lexeme: $message,
            line: $self.line,
        }
    };
}

pub struct Scanner<'a> {
    source: &'a [u8],
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub(crate) fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return token!(self, TokenType::EoF);
        }

        match self.advance() {
            b'(' => token!(self, TokenType::LeftParen),
            b')' => token!(self, TokenType::RightParen),
            b'{' => token!(self, TokenType::LeftBrace),
            b'}' => token!(self, TokenType::RightBrace),
            b';' => token!(self, TokenType::Semicolon),
            b',' => token!(self, TokenType::Comma),
            b'.' => token!(self, TokenType::Dot),
            b'-' => token!(self, TokenType::Minus),
            b'+' => token!(self, TokenType::Plus),
            b'/' => token!(self, TokenType::Slash),
            b'*' => token!(self, TokenType::Star),
            b'!' => {
                if self.match_advance(b'=') {
                    token!(self, TokenType::BangEqual)
                } else {
                    token!(self, TokenType::Bang)
                }
            }
            b'=' => {
                if self.match_advance(b'=') {
                    token!(self, TokenType::EqualEqual)
                } else {
                    token!(self, TokenType::Equal)
                }
            }
            b'<' => {
                if self.match_advance(b'=') {
                    token!(self, TokenType::LessEqual)
                } else {
                    token!(self, TokenType::Less)
                }
            }
            b'>' => {
                if self.match_advance(b'=') {
                    token!(self, TokenType::GreaterEqual)
                } else {
                    token!(self, TokenType::Greater)
                }
            }
            b'"' => self.string(),
            b if b.is_ascii_digit() => self.number(),
            a if a.is_ascii_alphabetic() => self.identifier(),
            b'_' => self.identifier(),
            _ => error_token!(self, "Unexpected character"),
        }
    }

    fn advance(&mut self) -> &u8 {
        self.current += 1;
        &self.source[self.current - 1]
    }

    fn match_advance(&mut self, c: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] == c {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn peek(&self) -> u8 {
        self.source[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.current += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error_token!(self, "Unterminated string")
        } else {
            self.advance();
            token!(self, TokenType::String)
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                return;
            }
            match self.peek() {
                b' ' | b'\t' | b'\r' => {
                    self.advance();
                }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                b'/' => {
                    if self.peek_next() == b'/' {
                        while self.peek() != b'\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        // fraction
        if self.peek() == b'.' {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        token!(self, TokenType::Number)
    }

    fn identifier(&mut self) -> Token {
        loop {
            let a = self.peek();
            if a.is_ascii_digit() || a.is_ascii_alphabetic() || a == b'_' {
                self.advance();
            } else {
                break;
            }
        }
        match self.source[self.start] {
            b'a' => self.keyword_if_match(1, 2, "nd", TokenType::And),
            b'c' => self.keyword_if_match(1, 4, "lass", TokenType::Class),
            b'e' => self.keyword_if_match(1, 3, "lse", TokenType::Else),
            b'i' => self.keyword_if_match(1, 1, "f", TokenType::If),
            b'n' => self.keyword_if_match(1, 2, "il", TokenType::Nil),
            b'o' => self.keyword_if_match(1, 1, "r", TokenType::Or),
            b'p' => self.keyword_if_match(1, 4, "rint", TokenType::Print),
            b'r' => self.keyword_if_match(1, 5, "eturn", TokenType::Return),
            b's' => self.keyword_if_match(1, 4, "uper", TokenType::Super),
            b'v' => self.keyword_if_match(1, 2, "ar", TokenType::Var),
            b'w' => self.keyword_if_match(1, 4, "hile", TokenType::While),
            b'f' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        b'a' => self.keyword_if_match(2, 3, "lse", TokenType::False),
                        b'o' => self.keyword_if_match(2, 1, "r", TokenType::For),
                        b'u' => self.keyword_if_match(2, 1, "n", TokenType::Fun),
                        _ => token!(self, TokenType::Identifier),
                    }
                } else {
                    token!(self, TokenType::Identifier)
                }
            }
            b't' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        b'h' => self.keyword_if_match(2, 3, "is", TokenType::This),
                        b'r' => self.keyword_if_match(2, 1, "ue", TokenType::True),
                        _ => token!(self, TokenType::Identifier),
                    }
                } else {
                    token!(self, TokenType::Identifier)
                }
            }
            _ => token!(self, TokenType::Identifier),
        }
    }

    fn keyword_if_match(
        &self,
        start: usize,
        length: usize,
        check: &str,
        ttype: TokenType,
    ) -> Token {
        if self.current - self.start == start + length
            && self.source[self.start + start..self.start + start + length] == *check.as_bytes()
        {
            token!(self, ttype)
        } else {
            token!(self, TokenType::Identifier)
        }
    }
}
