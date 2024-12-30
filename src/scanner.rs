use std::str::Chars;

use crate::{location::SourceLocation, token::*};

use itertools::{Itertools, MultiPeek};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected character `{c}` at {location}")]
    UnexpectedCharacter { c: char, location: SourceLocation },

    #[error("Unterminated string starting at {location}")]
    UnterminatedString { location: SourceLocation },

    #[error("Unterminated /* block comment */ starting at {location}")]
    UnterminatedComment { location: SourceLocation },
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

    pub fn scan(self) -> Result<Vec<TokenItem>, Vec<Error>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut location = SourceLocation::new(1, 0);
        let mut chars = self.source.chars().multipeek();
        while let Some(c) = chars.next() {
            let mut increment = 1;
            match c {
                '(' => tokens.push(TokenItem {
                    ttype: TokenType::Basic(BasicToken::LeftParen),
                    lexeme: c.to_string(),
                    literal: None,
                    location,
                }),
                ')' => tokens.push(TokenItem {
                    ttype: TokenType::Basic(BasicToken::RightParen),
                    lexeme: c.to_string(),
                    literal: None,
                    location,
                }),
                '{' => tokens.push(TokenItem {
                    ttype: TokenType::Basic(BasicToken::LeftBrace),
                    lexeme: c.to_string(),
                    literal: None,
                    location,
                }),
                '}' => tokens.push(TokenItem {
                    ttype: TokenType::Basic(BasicToken::RightBrace),
                    lexeme: c.to_string(),
                    literal: None,
                    location,
                }),
                ',' => tokens.push(TokenItem {
                    ttype: TokenType::Basic(BasicToken::Comma),
                    lexeme: c.to_string(),
                    literal: None,
                    location,
                }),
                '.' => tokens.push(TokenItem {
                    ttype: TokenType::Basic(BasicToken::Dot),
                    lexeme: c.to_string(),
                    literal: None,
                    location,
                }),
                '-' => tokens.push(TokenItem {
                    ttype: TokenType::Basic(BasicToken::Minus),
                    lexeme: c.to_string(),
                    literal: None,
                    location,
                }),
                '+' => tokens.push(TokenItem {
                    ttype: TokenType::Basic(BasicToken::Plus),
                    lexeme: c.to_string(),
                    literal: None,
                    location,
                }),
                ';' => tokens.push(TokenItem {
                    ttype: TokenType::Basic(BasicToken::Semicolon),
                    lexeme: c.to_string(),
                    literal: None,
                    location,
                }),
                '*' => tokens.push(TokenItem {
                    ttype: TokenType::Basic(BasicToken::Star),
                    lexeme: c.to_string(),
                    literal: None,
                    location,
                }),
                '!' => {
                    if matches!(chars.peek(), Some('=')) {
                        tokens.push(TokenItem {
                            ttype: TokenType::Basic(BasicToken::BangEq),
                            lexeme: format!("{}{}", c, chars.next().unwrap()),
                            literal: None,
                            location,
                        });
                        increment += 1;
                    } else {
                        tokens.push(TokenItem {
                            ttype: TokenType::Basic(BasicToken::Bang),
                            lexeme: c.to_string(),
                            literal: None,
                            location,
                        });
                    }
                }
                '=' => {
                    if matches!(chars.peek(), Some('=')) {
                        tokens.push(TokenItem {
                            ttype: TokenType::Basic(BasicToken::EqualEq),
                            lexeme: format!("{}{}", c, chars.next().unwrap()),
                            literal: None,
                            location,
                        });
                        increment += 1;
                    } else {
                        tokens.push(TokenItem {
                            ttype: TokenType::Basic(BasicToken::Equal),
                            lexeme: c.to_string(),
                            literal: None,
                            location,
                        });
                    }
                }
                '>' => {
                    if matches!(chars.peek(), Some('=')) {
                        tokens.push(TokenItem {
                            ttype: TokenType::Basic(BasicToken::GreaterEq),
                            lexeme: format!("{}{}", c, chars.next().unwrap()),
                            literal: None,
                            location,
                        });
                        increment += 1;
                    } else {
                        tokens.push(TokenItem {
                            ttype: TokenType::Basic(BasicToken::Greater),
                            lexeme: c.to_string(),
                            literal: None,
                            location,
                        });
                    }
                }
                '<' => {
                    if matches!(chars.peek(), Some('=')) {
                        tokens.push(TokenItem {
                            ttype: TokenType::Basic(BasicToken::LessEq),
                            lexeme: format!("{}{}", c, chars.next().unwrap()),
                            literal: None,
                            location,
                        });
                        increment += 1;
                    } else {
                        tokens.push(TokenItem {
                            ttype: TokenType::Basic(BasicToken::Less),
                            lexeme: c.to_string(),
                            literal: None,
                            location,
                        });
                    }
                }
                '/' => {
                    let c2 = chars.peek();
                    if matches!(c2, Some('/')) {
                        while !matches!(chars.peek(), Some('\n')) {
                            chars.next();
                            increment += 1;
                        }
                    } else if matches!(c2, Some('*')) {
                        if let Some(move_by) = Self::parse_multiline_comment(&mut chars) {
                            location.merge(move_by);
                            increment = 0;
                        } else {
                            errors.push(Error::UnterminatedComment { location });
                        }
                    } else {
                        tokens.push(TokenItem {
                            ttype: TokenType::Basic(BasicToken::Slash),
                            lexeme: c.to_string(),
                            literal: None,
                            location,
                        });
                    }
                }
                '"' => {
                    if let Some((string, move_by)) = Self::parse_string(c, &mut chars) {
                        let literal_string = string[1..string.len() - 1].to_string();
                        tokens.push(TokenItem {
                            ttype: TokenType::Literal(LiteralToken::String),
                            lexeme: string,
                            literal: Some(Literal::String(literal_string)),
                            location,
                        });
                        location.merge(move_by);
                        increment = 0;
                    } else {
                        errors.push(Error::UnterminatedString { location });
                    }
                }
                c if c.is_digit(10) => {
                    let (lexeme, num, add_increment) = Self::parse_number(c, &mut chars);
                    increment += add_increment;
                    tokens.push(TokenItem {
                        ttype: TokenType::Literal(LiteralToken::Number),
                        lexeme,
                        literal: Some(Literal::Number(num)),
                        location,
                    });
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let (string, add_increment) = Self::parse_varchar(c, &mut chars);
                    increment += add_increment;
                    let (ttype, literal) = match TokenType::from_string(&string) {
                        Some(ttype) => (ttype, None),
                        _ => (
                            TokenType::Literal(LiteralToken::Identifier),
                            Some(Literal::Identifier(string.clone())),
                        ),
                    };
                    tokens.push(TokenItem {
                        ttype,
                        lexeme: string,
                        literal,
                        location,
                    });
                }
                '\n' => {
                    location.newline();
                    increment = 0;
                }
                ' ' | '\r' | '\t' => {
                    // ignore whitespace
                }
                other => errors.push(Error::UnexpectedCharacter { c: other, location }),
            }
            location.advance_by(increment);
        }
        tokens.push(TokenItem {
            ttype: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            location,
        });
        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn parse_number(start: char, chars: &mut MultiPeek<Chars<'_>>) -> (String, f64, usize) {
        let mut lexeme = format!("{start}");
        let mut increment = 0;
        let mut has_dot = false;
        while let Some(c2) = chars.peek() {
            if c2.is_digit(10) {
                let c2 = chars.next().unwrap();
                increment += 1;
                lexeme.push(c2);
            } else if *c2 == '.' {
                if has_dot {
                    break;
                }
                let c3 = chars.peek();
                if !matches!(c3, Some(c4) if c4.is_digit(10)) {
                    break;
                }
                has_dot = true;
                let c2 = chars.next().unwrap();
                let c3 = chars.next().unwrap();
                increment += 2;
                lexeme.push(c2);
                lexeme.push(c3);
            } else {
                break;
            }
        }
        let num = lexeme.parse::<f64>().expect("Should be parseable float");
        (lexeme, num, increment)
    }

    fn parse_varchar(start: char, chars: &mut MultiPeek<Chars<'_>>) -> (String, usize) {
        let mut string = format!("{start}");
        let mut increment = 0;
        while let Some(c2) = chars.peek() {
            if !(c2.is_ascii_alphabetic() || c2.is_digit(10) || *c2 == '_') {
                break;
            }
            let c2 = chars.next().unwrap();
            increment += 1;
            string.push(c2);
        }
        (string, increment)
    }

    fn parse_string(
        start: char,
        chars: &mut MultiPeek<Chars<'_>>,
    ) -> Option<(String, SourceLocation)> {
        let mut string = format!("{start}");
        let mut move_by = SourceLocation::new(0, 0);
        let mut increment = 1;
        loop {
            let ctest = chars.next();
            increment += 1;
            match ctest {
                Some(c2) => {
                    string.push(c2);
                    if matches!(c2, '"') {
                        move_by.advance_by(increment);
                        return Some((string, move_by));
                    } else {
                        if matches!(c2, '\n') {
                            move_by.newline();
                            increment = 0;
                        }
                    }
                }
                None => return None,
            }
        }
    }

    fn parse_multiline_comment(chars: &mut MultiPeek<Chars<'_>>) -> Option<SourceLocation> {
        let mut move_by = SourceLocation::new(0, 0);
        let mut increment = 1;
        // dept of comment nesting
        let mut comment_level = 1;
        while let Some(c2) = chars.next() {
            increment += 1;
            if matches!(c2, '/') && matches!(chars.peek(), Some('*')) {
                chars.next();
                increment += 1;
                comment_level += 1;
            } else if matches!(c2, '*') && matches!(chars.peek(), Some('/')) {
                chars.next();
                increment += 1;
                comment_level -= 1;
                if comment_level == 0 {
                    break;
                }
            } else if matches!(c2, '\n') {
                move_by.newline();
                increment = 0;
            }
        }
        move_by.advance_by(increment);
        if comment_level == 0 {
            Some(move_by)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scanner() {
        let scanner = Scanner::new("var x = 5;");
        let tokens = scanner.scan().unwrap();
        assert_eq!(tokens, vec![
            TokenItem {
                ttype: TokenType::KeywordToken(KeywordToken::Var),
                lexeme: "var".to_string(),
                literal: None,
                location: SourceLocation::new(1, 0)
            },
            TokenItem {
                ttype: TokenType::Literal(LiteralToken::Identifier),
                lexeme: "x".to_string(),
                literal: Some(Literal::Identifier("x".to_string())),
                location: SourceLocation::new(1, 4)
            },
            TokenItem {
                ttype: TokenType::Basic(BasicToken::Equal),
                lexeme: "=".to_string(),
                literal: None,
                location: SourceLocation::new(1, 6)
            },
            TokenItem {
                ttype: TokenType::Literal(LiteralToken::Number),
                lexeme: "5".to_string(),
                literal: Some(Literal::Number(5.0)),
                location: SourceLocation::new(1, 8)
            },
            TokenItem {
                ttype: TokenType::Basic(BasicToken::Semicolon),
                lexeme: ";".to_string(),
                literal: None,
                location: SourceLocation::new(1, 9)
            },
            TokenItem {
                ttype: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                location: SourceLocation::new(1, 10)
            }
        ]);
    }

    #[test]
    fn test_scanner_number() {
        let scanner = Scanner::new("var x = 5.5;");
        let tokens = scanner.scan().unwrap();
        assert_eq!(tokens, vec![
            TokenItem {
                ttype: TokenType::KeywordToken(KeywordToken::Var),
                lexeme: "var".to_string(),
                literal: None,
                location: SourceLocation::new(1, 0)
            },
            TokenItem {
                ttype: TokenType::Literal(LiteralToken::Identifier),
                lexeme: "x".to_string(),
                literal: Some(Literal::Identifier("x".to_string())),
                location: SourceLocation::new(1, 4)
            },
            TokenItem {
                ttype: TokenType::Basic(BasicToken::Equal),
                lexeme: "=".to_string(),
                literal: None,
                location: SourceLocation::new(1, 6)
            },
            TokenItem {
                ttype: TokenType::Literal(LiteralToken::Number),
                lexeme: "5.5".to_string(),
                literal: Some(Literal::Number(5.5)),
                location: SourceLocation::new(1, 8)
            },
            TokenItem {
                ttype: TokenType::Basic(BasicToken::Semicolon),
                lexeme: ";".to_string(),
                literal: None,
                location: SourceLocation::new(1, 11)
            },
            TokenItem {
                ttype: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                location: SourceLocation::new(1, 12)
            }
        ]);
        let scanner = Scanner::new("var x = 5.5.5;");
        let tokens = scanner.scan().unwrap();
        assert_eq!(tokens, vec![
            TokenItem {
                ttype: TokenType::KeywordToken(KeywordToken::Var),
                lexeme: "var".to_string(),
                literal: None,
                location: SourceLocation::new(1, 0)
            },
            TokenItem {
                ttype: TokenType::Literal(LiteralToken::Identifier),
                lexeme: "x".to_string(),
                literal: Some(Literal::Identifier("x".to_string())),
                location: SourceLocation::new(1, 4)
            },
            TokenItem {
                ttype: TokenType::Basic(BasicToken::Equal),
                lexeme: "=".to_string(),
                literal: None,
                location: SourceLocation::new(1, 6)
            },
            TokenItem {
                ttype: TokenType::Literal(LiteralToken::Number),
                lexeme: "5.5".to_string(),
                literal: Some(Literal::Number(5.5)),
                location: SourceLocation::new(1, 8)
            },
            TokenItem {
                ttype: TokenType::Basic(BasicToken::Dot),
                lexeme: ".".to_string(),
                literal: None,
                location: SourceLocation::new(1, 11)
            },
            TokenItem {
                ttype: TokenType::Literal(LiteralToken::Number),
                lexeme: "5".to_string(),
                literal: Some(Literal::Number(5.0)),
                location: SourceLocation::new(1, 12)
            },
            TokenItem {
                ttype: TokenType::Basic(BasicToken::Semicolon),
                lexeme: ";".to_string(),
                literal: None,
                location: SourceLocation::new(1, 13)
            },
            TokenItem {
                ttype: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                location: SourceLocation::new(1, 14)
            }
        ]);
    }

    #[test]
    fn test_scanner_multiline_comment() {
        let scanner = Scanner::new("/* /* this is a\n multiline */ comment */hello");
        let tokens = scanner.scan().unwrap();
        assert_eq!(tokens, vec![
            TokenItem {
                ttype: TokenType::Literal(LiteralToken::Identifier),
                lexeme: "hello".to_string(),
                literal: Some(Literal::Identifier("hello".to_string())),
                location: SourceLocation::new(2, 24)
            },
            TokenItem {
                ttype: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                location: SourceLocation::new(2, 29)
            }
        ]);
    }

    #[test]
    fn test_scanner_string() {
        let scanner = Scanner::new("var x = \"hello world\";");
        let tokens = scanner.scan().unwrap();
        assert_eq!(tokens, vec![
            TokenItem {
                ttype: TokenType::KeywordToken(KeywordToken::Var),
                lexeme: "var".to_string(),
                literal: None,
                location: SourceLocation::new(1, 0)
            },
            TokenItem {
                ttype: TokenType::Literal(LiteralToken::Identifier),
                lexeme: "x".to_string(),
                literal: Some(Literal::Identifier("x".to_string())),
                location: SourceLocation::new(1, 4)
            },
            TokenItem {
                ttype: TokenType::Basic(BasicToken::Equal),
                lexeme: "=".to_string(),
                literal: None,
                location: SourceLocation::new(1, 6)
            },
            TokenItem {
                ttype: TokenType::Literal(LiteralToken::String),
                lexeme: "\"hello world\"".to_string(),
                literal: Some(Literal::String("hello world".to_string())),
                location: SourceLocation::new(1, 8)
            },
            TokenItem {
                ttype: TokenType::Basic(BasicToken::Semicolon),
                lexeme: ";".to_string(),
                literal: None,
                location: SourceLocation::new(1, 21)
            },
            TokenItem {
                ttype: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                location: SourceLocation::new(1, 22)
            }
        ]);
        let scanner = Scanner::new("\"hello\nworld\"");
        let tokens = scanner.scan().unwrap();
        assert_eq!(tokens, vec![
            TokenItem {
                ttype: TokenType::Literal(LiteralToken::String),
                lexeme: "\"hello\nworld\"".to_string(),
                literal: Some(Literal::String("hello\nworld".to_string())),
                location: SourceLocation::new(1, 0)
            },
            TokenItem {
                ttype: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                location: SourceLocation::new(2, 6)
            }
        ]);
    }
}
