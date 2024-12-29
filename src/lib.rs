use std::default;

#[derive(Debug, Copy, Clone)]
enum BasicToken {
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
enum KeywordToken {
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
enum LiteralToken {
    Identifier,
    String,
    Number,
}

#[derive(Debug, Clone)]
enum Literal {
    Identifier(String),
    String(String),
    Number(usize, usize),
}

#[derive(Debug, Copy, Clone)]
enum TokenType {
    Basic(BasicToken),
    KeywordToken(KeywordToken),
    Literal(LiteralToken),
    EOF,
}

struct TokenItem {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
    pos: usize,
}

impl TokenItem {
    fn to_string(&self) -> String {
        format!("{:?} {:?} {:?}", self.ttype, self.lexeme, self.literal)
    }
}

struct Scanner {
    source: String,
    tokens: Vec<TokenItem>,
}

impl Scanner {
    fn new(input: &str) -> Scanner {
        Self {
            source: input.to_owned(),
            tokens: Vec::new(),
        }
    }

    fn scan(&mut self) {
        let mut lines = self.source.lines().enumerate();
        'outer: while let Some((mut line, l)) = lines.next() {
            let mut chars = l.chars().enumerate().peekable();
            while let Some((pos, c)) = chars.next() {
                match c {
                    '(' => self.tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::LeftParen),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    ')' => self.tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::RightParen),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '{' => self.tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::LeftBrace),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '}' => self.tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::RightBrace),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    ',' => self.tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Comma),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '.' => self.tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Dot),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '-' => self.tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Minus),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '+' => self.tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Plus),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    ';' => self.tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Semicolon),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '*' => self.tokens.push(TokenItem {
                        ttype: TokenType::Basic(BasicToken::Star),
                        lexeme: c.to_string(),
                        literal: None,
                        line,
                        pos,
                    }),
                    '!' => {
                        if matches!(chars.peek(), Some(&(_, '='))) {
                            self.tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::BangEq),
                                lexeme: [c, chars.next().unwrap().1].into_iter().collect(),
                                literal: None,
                                line,
                                pos,
                            })
                        } else {
                            self.tokens.push(TokenItem {
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
                            self.tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::EqualEq),
                                lexeme: format!("{}{}", c, chars.next().unwrap().1),
                                literal: None,
                                line,
                                pos,
                            })
                        } else {
                            self.tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::EqualEq),
                                lexeme: c.to_string(),
                                literal: None,
                                line,
                                pos,
                            })
                        }
                    }
                    '>' => {
                        if matches!(chars.peek(), Some(&(_, '='))) {
                            self.tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::GreaterEq),
                                lexeme: format!("{}{}", c, chars.next().unwrap().1),
                                literal: None,
                                line,
                                pos,
                            })
                        } else {
                            self.tokens.push(TokenItem {
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
                            self.tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::LessEq),
                                lexeme: format!("{}{}", c, chars.next().unwrap().1),
                                literal: None,
                                line,
                                pos,
                            })
                        } else {
                            self.tokens.push(TokenItem {
                                ttype: TokenType::Basic(BasicToken::Less),
                                lexeme: c.to_string(),
                                literal: None,
                                line,
                                pos,
                            })
                        }
                    }
                    '/' => {
                        if matches!(chars.peek(), Some(&(_, '='))) {
                            continue 'outer;
                        } else {
                            self.tokens.push(TokenItem {
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
                                        self.tokens.push(TokenItem {
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
                                        chars = new_l.chars().enumerate().peekable();
                                        line = new_line;
                                    }
                                    None => println!("Unterminated string"),
                                },
                            }
                        }
                    }
                    c if c.is_digit(10) => {
                        // TODO !! Numbe literals
                    }
                    ' ' | '\r' | '\t' => {
                        // ignore whitespace
                    }
                    _ => println!("Unexpected character '{c}' at {line}:{pos}"),
                }
            }
        }
        todo!()
    }
}
