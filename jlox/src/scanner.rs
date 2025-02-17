use std::str::CharIndices;

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

trait Offset {
    fn offset(&mut self, max: usize) -> usize;
}

impl Offset for MultiPeek<CharIndices<'_>> {
    fn offset(&mut self, max: usize) -> usize {
        self.peek().map(|(i, _)| *i).unwrap_or(max)
    }
}

pub struct Scanner {}

impl Scanner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn scan(self, input: &'static str) -> Result<Vec<TokenItem>, Vec<Error>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut location = SourceLocation::new(1, 0);
        let mut chars = input.char_indices().multipeek();
        let max = input.len();
        let basic_token =
            |ttype: TokenType, lexeme: &'static str, location: SourceLocation| TokenItem {
                ttype,
                lexeme,
                literal: None,
                location,
            };
        while let Some(ci) = chars.next() {
            let mut increment = 1;
            match ci.1 {
                '(' => tokens.push(basic_token(
                    TokenType::LeftParen,
                    &input[ci.0..chars.offset(max)],
                    location,
                )),
                ')' => tokens.push(basic_token(
                    TokenType::RightParen,
                    &input[ci.0..chars.offset(max)],
                    location,
                )),
                '{' => tokens.push(basic_token(
                    TokenType::LeftBrace,
                    &input[ci.0..chars.offset(max)],
                    location,
                )),
                '}' => tokens.push(basic_token(
                    TokenType::RightBrace,
                    &input[ci.0..chars.offset(max)],
                    location,
                )),
                ',' => tokens.push(basic_token(
                    TokenType::Comma,
                    &input[ci.0..chars.offset(max)],
                    location,
                )),
                '.' => tokens.push(basic_token(
                    TokenType::Dot,
                    &input[ci.0..chars.offset(max)],
                    location,
                )),
                '-' => tokens.push(basic_token(
                    TokenType::Minus,
                    &input[ci.0..chars.offset(max)],
                    location,
                )),
                '+' => tokens.push(basic_token(
                    TokenType::Plus,
                    &input[ci.0..chars.offset(max)],
                    location,
                )),
                ';' => tokens.push(basic_token(
                    TokenType::Semicolon,
                    &input[ci.0..chars.offset(max)],
                    location,
                )),
                '*' => tokens.push(basic_token(
                    TokenType::Star,
                    &input[ci.0..chars.offset(max)],
                    location,
                )),
                '!' => {
                    let c2 = chars.peek();
                    match c2 {
                        Some((_, '=')) => {
                            let _ = chars.next();
                            tokens.push(basic_token(
                                TokenType::BangEq,
                                &input[ci.0..chars.offset(max)],
                                location,
                            ));
                            increment += 1;
                        }
                        _ => tokens.push(basic_token(
                            TokenType::Bang,
                            &input[ci.0..c2.map(|(i, _)| *i).unwrap_or(max)],
                            location,
                        )),
                    }
                }
                '=' => {
                    let c2 = chars.peek();
                    match c2 {
                        Some((_, '=')) => {
                            let _ = chars.next();
                            tokens.push(basic_token(
                                TokenType::EqualEq,
                                &input[ci.0..chars.offset(max)],
                                location,
                            ));
                            increment += 1;
                        }
                        _ => tokens.push(basic_token(
                            TokenType::Equal,
                            &input[ci.0..c2.map(|(i, _)| *i).unwrap_or(max)],
                            location,
                        )),
                    }
                }
                '>' => {
                    let c2 = chars.peek();
                    match c2 {
                        Some((_, '=')) => {
                            let _ = chars.next();
                            tokens.push(basic_token(
                                TokenType::GreaterEq,
                                &input[ci.0..chars.offset(max)],
                                location,
                            ));
                            increment += 1;
                        }
                        _ => tokens.push(basic_token(
                            TokenType::Greater,
                            &input[ci.0..c2.map(|(i, _)| *i).unwrap_or(max)],
                            location,
                        )),
                    }
                }
                '<' => {
                    let c2 = chars.peek();
                    match c2 {
                        Some((_, '=')) => {
                            let _ = chars.next();
                            tokens.push(basic_token(
                                TokenType::LessEq,
                                &input[ci.0..chars.offset(max)],
                                location,
                            ));
                            increment += 1;
                        }
                        _ => tokens.push(basic_token(
                            TokenType::Less,
                            &input[ci.0..c2.map(|(i, _)| *i).unwrap_or(max)],
                            location,
                        )),
                    }
                }
                '/' => {
                    let c2 = chars.peek();
                    if matches!(c2, Some((_, '/'))) {
                        while !matches!(chars.peek(), Some((_, '\n')) | None) {
                            chars.next();
                            increment += 1;
                        }
                    } else if matches!(c2, Some((_, '*'))) {
                        if let Some(move_by) = Self::parse_multiline_comment(&mut chars) {
                            location.merge(move_by);
                            increment = 0;
                        } else {
                            errors.push(Error::UnterminatedComment { location });
                        }
                    } else {
                        tokens.push(basic_token(
                            TokenType::Slash,
                            &input[ci.0..c2.map(|(i, _)| *i).unwrap_or(max)],
                            location,
                        ));
                    }
                }
                '"' => {
                    if let Some((string, move_by)) = Self::parse_string(&mut chars) {
                        tokens.push(TokenItem {
                            ttype: TokenType::String,
                            lexeme: &input[ci.0..chars.offset(max)],
                            literal: Some(Literal::String(string.into())),
                            location,
                        });
                        location.merge(move_by);
                        increment = 0;
                    } else {
                        errors.push(Error::UnterminatedString { location });
                    }
                }
                c if c.is_ascii_digit() => {
                    let (end, add_increment) = Self::parse_number(max, &mut chars);
                    let lexeme = &input[ci.0..end];
                    let num = lexeme.parse().unwrap();
                    increment += add_increment;
                    tokens.push(TokenItem {
                        ttype: TokenType::Number,
                        lexeme,
                        literal: Some(Literal::Number(num)),
                        location,
                    });
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let (end, add_increment) = Self::parse_varchar(max, &mut chars);
                    let lexeme = &input[ci.0..end];
                    increment += add_increment;
                    let (ttype, literal) = match TokenType::from_string(lexeme) {
                        Some(TokenType::True) => (TokenType::True, Some(Literal::True)),
                        Some(TokenType::False) => (TokenType::False, Some(Literal::False)),
                        Some(TokenType::Nil) => (TokenType::Nil, Some(Literal::Nil)),
                        Some(ttype) => (ttype, None),
                        _ => (TokenType::Identifier, None),
                    };
                    tokens.push(TokenItem {
                        ttype,
                        lexeme,
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
            ttype: TokenType::EoF,
            lexeme: "",
            literal: None,
            location,
        });
        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn parse_number(max: usize, chars: &mut MultiPeek<CharIndices<'_>>) -> (usize, usize) {
        let mut increment = 0;
        let mut has_dot = false;
        let mut end;
        loop {
            let c2 = chars.peek();
            let Some(c2) = c2 else {
                end = max;
                break;
            };
            end = c2.0;
            if c2.1.is_ascii_digit() {
                let _ = chars.next().unwrap();
                increment += 1;
            } else if c2.1 == '.' {
                if has_dot {
                    break;
                }
                let c3 = chars.peek();
                if !matches!(c3, Some((_, c4)) if c4.is_ascii_digit()) {
                    break;
                }
                has_dot = true;
                let _ = chars.next().unwrap();
                let _ = chars.next().unwrap();
                increment += 2;
            } else {
                break;
            }
        }
        (end, increment)
    }

    fn parse_varchar(max: usize, chars: &mut MultiPeek<CharIndices<'_>>) -> (usize, usize) {
        let mut increment = 0;
        let mut end;
        loop {
            let c2 = chars.peek();
            let Some(c2) = c2 else {
                end = max;
                break;
            };
            end = c2.0;
            if !(c2.1.is_ascii_alphabetic() || c2.1.is_ascii_digit() || c2.1 == '_') {
                break;
            }
            let _ = chars.next().unwrap();
            increment += 1;
        }
        (end, increment)
    }

    fn parse_string(chars: &mut MultiPeek<CharIndices<'_>>) -> Option<(String, SourceLocation)> {
        let mut string = String::new();
        let mut move_by = SourceLocation::new(0, 0);
        let mut increment = 1;
        loop {
            let ctest = chars.next();
            increment += 1;
            match ctest {
                Some((_, c2)) => {
                    if matches!(c2, '"') {
                        move_by.advance_by(increment);
                        return Some((string, move_by));
                    } else if matches!(c2, '\n') {
                        move_by.newline();
                        increment = 0;
                    }
                    string.push(c2);
                }
                None => return None,
            }
        }
    }

    fn parse_multiline_comment(chars: &mut MultiPeek<CharIndices<'_>>) -> Option<SourceLocation> {
        let mut move_by = SourceLocation::new(0, 0);
        let mut increment = 1;
        // dept of comment nesting
        let mut comment_level = 1;
        while let Some(c2) = chars.next() {
            increment += 1;
            if matches!(c2.1, '/') && matches!(chars.peek(), Some((_, '*'))) {
                chars.next();
                increment += 1;
                comment_level += 1;
            } else if matches!(c2.1, '*') && matches!(chars.peek(), Some((_, '/'))) {
                chars.next();
                increment += 1;
                comment_level -= 1;
                if comment_level == 0 {
                    break;
                }
            } else if matches!(c2.1, '\n') {
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
        let tokens = Scanner::new().scan("var x = 5;").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenItem {
                    ttype: TokenType::Var,
                    lexeme: "var",
                    literal: None,
                    location: SourceLocation::new(1, 0)
                },
                TokenItem {
                    ttype: TokenType::Identifier,
                    lexeme: "x",
                    literal: None,
                    location: SourceLocation::new(1, 4)
                },
                TokenItem {
                    ttype: TokenType::Equal,
                    lexeme: "=",
                    literal: None,
                    location: SourceLocation::new(1, 6)
                },
                TokenItem {
                    ttype: TokenType::Number,
                    lexeme: "5",
                    literal: Some(Literal::Number(5.0)),
                    location: SourceLocation::new(1, 8)
                },
                TokenItem {
                    ttype: TokenType::Semicolon,
                    lexeme: ";",
                    literal: None,
                    location: SourceLocation::new(1, 9)
                },
                TokenItem {
                    ttype: TokenType::EoF,
                    lexeme: "",
                    literal: None,
                    location: SourceLocation::new(1, 10)
                }
            ]
        );
    }

    #[test]
    fn test_scanner_number() {
        let tokens = Scanner::new().scan("var x = 5.5;").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenItem {
                    ttype: TokenType::Var,
                    lexeme: "var",
                    literal: None,
                    location: SourceLocation::new(1, 0)
                },
                TokenItem {
                    ttype: TokenType::Identifier,
                    lexeme: "x",
                    literal: None,
                    location: SourceLocation::new(1, 4)
                },
                TokenItem {
                    ttype: TokenType::Equal,
                    lexeme: "=",
                    literal: None,
                    location: SourceLocation::new(1, 6)
                },
                TokenItem {
                    ttype: TokenType::Number,
                    lexeme: "5.5",
                    literal: Some(Literal::Number(5.5)),
                    location: SourceLocation::new(1, 8)
                },
                TokenItem {
                    ttype: TokenType::Semicolon,
                    lexeme: ";",
                    literal: None,
                    location: SourceLocation::new(1, 11)
                },
                TokenItem {
                    ttype: TokenType::EoF,
                    lexeme: "",
                    literal: None,
                    location: SourceLocation::new(1, 12)
                }
            ]
        );
        let tokens = Scanner::new().scan("var x = 5.5.5;").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenItem {
                    ttype: TokenType::Var,
                    lexeme: "var",
                    literal: None,
                    location: SourceLocation::new(1, 0)
                },
                TokenItem {
                    ttype: TokenType::Identifier,
                    lexeme: "x",
                    literal: None,
                    location: SourceLocation::new(1, 4)
                },
                TokenItem {
                    ttype: TokenType::Equal,
                    lexeme: "=",
                    literal: None,
                    location: SourceLocation::new(1, 6)
                },
                TokenItem {
                    ttype: TokenType::Number,
                    lexeme: "5.5",
                    literal: Some(Literal::Number(5.5)),
                    location: SourceLocation::new(1, 8)
                },
                TokenItem {
                    ttype: TokenType::Dot,
                    lexeme: ".",
                    literal: None,
                    location: SourceLocation::new(1, 11)
                },
                TokenItem {
                    ttype: TokenType::Number,
                    lexeme: "5",
                    literal: Some(Literal::Number(5.0)),
                    location: SourceLocation::new(1, 12)
                },
                TokenItem {
                    ttype: TokenType::Semicolon,
                    lexeme: ";",
                    literal: None,
                    location: SourceLocation::new(1, 13)
                },
                TokenItem {
                    ttype: TokenType::EoF,
                    lexeme: "",
                    literal: None,
                    location: SourceLocation::new(1, 14)
                }
            ]
        );
    }

    #[test]
    fn test_scanner_multiline_comment() {
        let tokens = Scanner::new()
            .scan("/* /* this is a\n multiline */ comment */hello")
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenItem {
                    ttype: TokenType::Identifier,
                    lexeme: "hello",
                    literal: None,
                    location: SourceLocation::new(2, 24)
                },
                TokenItem {
                    ttype: TokenType::EoF,
                    lexeme: "",
                    literal: None,
                    location: SourceLocation::new(2, 29)
                }
            ]
        );
    }

    #[test]
    fn test_scanner_string() {
        let tokens = Scanner::new().scan("var x = \"hello world\";").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenItem {
                    ttype: TokenType::Var,
                    lexeme: "var",
                    literal: None,
                    location: SourceLocation::new(1, 0)
                },
                TokenItem {
                    ttype: TokenType::Identifier,
                    lexeme: "x",
                    literal: None,
                    location: SourceLocation::new(1, 4)
                },
                TokenItem {
                    ttype: TokenType::Equal,
                    lexeme: "=",
                    literal: None,
                    location: SourceLocation::new(1, 6)
                },
                TokenItem {
                    ttype: TokenType::String,
                    lexeme: "\"hello world\"",
                    literal: Some(Literal::String("hello world".to_string().into())),
                    location: SourceLocation::new(1, 8)
                },
                TokenItem {
                    ttype: TokenType::Semicolon,
                    lexeme: ";",
                    literal: None,
                    location: SourceLocation::new(1, 21)
                },
                TokenItem {
                    ttype: TokenType::EoF,
                    lexeme: "",
                    literal: None,
                    location: SourceLocation::new(1, 22)
                }
            ]
        );
        let tokens = Scanner::new().scan("\"hello\nworld\"").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenItem {
                    ttype: TokenType::String,
                    lexeme: "\"hello\nworld\"",
                    literal: Some(Literal::String("hello\nworld".to_string().into())),
                    location: SourceLocation::new(1, 0)
                },
                TokenItem {
                    ttype: TokenType::EoF,
                    lexeme: "",
                    literal: None,
                    location: SourceLocation::new(2, 6)
                }
            ]
        );
    }
}
