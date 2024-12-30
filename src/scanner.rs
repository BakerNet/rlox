use crate::Error;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum LiteralToken {
    Identifier,
    String,
    Number,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
}

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    Basic(BasicToken),
    KeywordToken(KeywordToken),
    Literal(LiteralToken),
    EOF,
}

#[derive(Debug)]
pub struct TokenItem {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
    pos: usize,
}

impl TokenItem {
    pub fn to_string(&self) -> String {
        format!("{:?} {:?} {:?}", self.ttype, self.lexeme, self.literal)
    }
}

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(input: &str) -> Scanner {
        Self {
            source: input.to_owned(),
        }
    }

    pub fn scan(self) -> Result<Vec<TokenItem>, Error> {
        let mut tokens = Vec::new();
        let mut lines = self.source.lines().enumerate();
        'outer: while let Some((line, l)) = lines.next() {
            let mut line = line + 1;
            let mut chars = l.chars().enumerate().peekable();
            while let Some((pos, c)) = chars.next() {
                match c {
                    '(' => tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::LeftParen),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    ')' => tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::RightParen),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '{' => tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::LeftBrace),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '}' => tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::RightBrace),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    ',' => tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Comma),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '.' => tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Dot),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '-' => tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Minus),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '+' => tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Plus),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    ';' => tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Semicolon),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '*' => tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Star),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '!' => {
                        if matches!(chars.peek(), Some(&(_, '='))) {
                            tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::BangEq),
                                lexeme: [c, chars.next().unwrap().1].into_iter().collect(),
                                literal: None,
                                line,
                                pos,
                            })
                        } else {
                            tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::Bang),
                                lexeme: c.to_string(),
                                literal: None,
                                line,
                                pos,
                            })
                        }
                    }
                    '=' => {
                        if matches!(chars.peek(), Some(&(_, '='))) {
                            tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::EqualEq),
                                lexeme: format!("{}{}", c, chars.next().unwrap().1),
                                literal: None,
                                line,
                                pos,
                            })
                        } else {
                            tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::Equal),
                                lexeme: c.to_string(),
                                literal: None,
                                line,
                                pos,
                            })
                        }
                    }
                    '>' => {
                        if matches!(chars.peek(), Some(&(_, '='))) {
                            tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::GreaterEq),
                                lexeme: format!("{}{}", c, chars.next().unwrap().1),
                                literal: None,
                                line,
                                pos,
                            })
                        } else {
                            tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::Greater),
                                lexeme: c.to_string(),
                                literal: None,
                                line,
                                pos,
                            })
                        }
                    }
                    '<' => {
                        if matches!(chars.peek(), Some(&(_, '='))) {
                            tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::LessEq),
                                lexeme: format!("{}{}", c, chars.next().unwrap().1),
                                literal: None,
                                line,
                                pos,
                            })
                        } else {
                            tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::Less),
                                lexeme: c.to_string(),
                                literal: None,
                                line,
                                pos,
                            })
                        }
                    }
                    '/' => {
                        if matches!(chars.peek(), Some(&(_, '/'))) {
                            continue 'outer;
                        } else {
                            tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::Slash),
                                lexeme: c.to_string(),
                                literal: None,
                                line,
                                pos,
                            })
                        }
                    }
                    '"' => {
                        let original_line = line;
                        let mut string = String::new();
                        loop {
                            let ctest = chars.next();
                            match ctest {
                                Some((_, c2)) => {
                                    if matches!(c2, '"') {
                                        tokens.push(TokenItem {
                                            ttype: TokenType::Literal(LiteralToken::String),
                                            lexeme: format!("\"{}\"", string),
                                            literal: Some(Literal::String(string)),
                                            line: original_line,
                                            pos: pos + 1,
                                        });
                                        break;
                                    } else {
                                        string.push(c2);
                                    }
                                }
                                None => match lines.next() {
                                    Some((new_line, new_l)) => {
                                        string.push('\n');
                                        chars = new_l.chars().enumerate().peekable();
                                        line = new_line;
                                    }
                                    None => {
                                        return Err(Error::Scanner());
                                    }
                                },
                            }
                        }
                    }
                    c if c.is_digit(10) => {
                        let mut lexeme = format!("{c}");
                        let mut is_dec = false;
                        let mut has_dot = false;
                        while let Some((_, c2)) = chars.peek() {
                            if c2.is_digit(10) {
                                let c2 = chars.next().unwrap().1;
                                lexeme.push(c2);
                            } else if *c2 == '.' {
                                if is_dec {
                                    // second dot
                                    break;
                                }
                                let c2 = chars.next().unwrap().1;
                                has_dot = true;
                                if let Some((_, c3)) = chars.peek() {
                                    if c3.is_digit(10) {
                                        is_dec = true;
                                        lexeme.push(c2);
                                    } else {
                                        // trailing dot
                                        break;
                                    }
                                } else {
                                    // dot at end of line
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        let num = lexeme.parse::<f64>().expect("Should be parseable float");
                        tokens.push(TokenItem {
                            ttype: TokenType::Literal(LiteralToken::Number),
                            lexeme,
                            literal: Some(Literal::Number(num)),
                            line,
                            pos,
                        });
                        if has_dot && !is_dec {
                            tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::Dot),
                                lexeme: ".".to_string(),
                                literal: None,
                                line,
                                pos,
                            });
                        }
                    }
                    c if c.is_ascii_alphabetic() || c == '_' => {
                        let mut string = format!("{c}");
                        while let Some((_, c2)) = chars.peek() {
                            if !(c2.is_ascii_alphabetic() || c2.is_digit(10) || *c2 == '_') {
                                break;
                            }
                            let c2 = chars.next().unwrap().1;
                            string.push(c2);
                        }
                        let (ttype, literal) = match string.as_str() {
                            "and" => (TokenType::KeywordToken(KeywordToken::And), None),
                            "class" => (TokenType::KeywordToken(KeywordToken::Class), None),
                            "else" => (TokenType::KeywordToken(KeywordToken::Else), None),
                            "false" => (TokenType::KeywordToken(KeywordToken::False), None),
                            "fun" => (TokenType::KeywordToken(KeywordToken::Fun), None),
                            "for" => (TokenType::KeywordToken(KeywordToken::For), None),
                            "if" => (TokenType::KeywordToken(KeywordToken::If), None),
                            "nil" => (TokenType::KeywordToken(KeywordToken::Nil), None),
                            "or" => (TokenType::KeywordToken(KeywordToken::Or), None),
                            "print" => (TokenType::KeywordToken(KeywordToken::Print), None),
                            "return" => (TokenType::KeywordToken(KeywordToken::Return), None),
                            "super" => (TokenType::KeywordToken(KeywordToken::Super), None),
                            "this" => (TokenType::KeywordToken(KeywordToken::This), None),
                            "true" => (TokenType::KeywordToken(KeywordToken::True), None),
                            "var" => (TokenType::KeywordToken(KeywordToken::Var), None),
                            "while" => (TokenType::KeywordToken(KeywordToken::While), None),
                            _ => (
                                TokenType::Literal(LiteralToken::Identifier),
                                Some(Literal::Identifier(string.clone())),
                            ),
                        };
                        tokens.push(TokenItem {
                            ttype,
                            lexeme: string,
                            literal,
                            line,
                            pos,
                        });
                    }
                    ' ' | '\r' | '\t' => {
                        // ignore whitespace
                    }
                    _ => println!("Unexpected character '{c}' at {line}:{pos}"),
                }
            }
        }
        Ok(tokens)
    }
}
