use crate::{
    location::SourceLocation,
    token::{BasicToken, KeywordToken, Literal, LiteralToken, TokenItem, TokenType},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("TODO error at {location}")]
    TODO { location: SourceLocation },

    #[error("Expected ')' after expression at {location}")]
    UnterminatedParen { location: SourceLocation },

    #[error("Unexpected token '{lexeme}'.  Expected expression at {location}")]
    UnexpectedToken {
        lexeme: String,
        location: SourceLocation,
    },
}

#[derive(Debug, Clone)]
pub struct Operator {
    pub ttype: TokenType,
    pub location: SourceLocation,
}

impl From<&TokenItem> for Operator {
    fn from(token: &TokenItem) -> Self {
        Self {
            ttype: token.ttype,
            location: token.location,
        }
    }
}

#[derive(Debug)]
pub enum AstNode {
    Binary {
        left: Box<AstNode>,
        operator: Operator,
        right: Box<AstNode>,
    },
    Unary {
        operator: Operator,
        right: Box<AstNode>,
    },
    Literal {
        value: Literal,
    },
}

// For chapter 6, we will only parse equality expressions.
pub struct Parser {
    source: Vec<TokenItem>,
}

impl Parser {
    pub fn new(source: Vec<TokenItem>) -> Self {
        Self { source }
    }

    pub fn parse(self) -> Result<AstNode, Vec<Error>> {
        // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
        // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
        // term           → factor ( ( "-" | "+" ) factor )* ;
        // factor         → unary ( ( "/" | "*" ) unary )* ;
        // unary          → ( "!" | "-" ) unary | primary ;
        // primary        → "true" | "false" | "nil"
        //                | NUMBER | STRING | "(" expression ")"
        self.equality(&self.source[..self.source.len() - 1], 0)
            .0
            .map_err(|e| vec![e].into())
    }

    fn equality(&self, tokens: &[TokenItem], cursor: usize) -> (Result<AstNode, Error>, usize) {
        // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
        let (try_left, mut new_cursor) = self.comparison(tokens, cursor);
        let mut left = if let Ok(left) = try_left {
            left
        } else {
            return (try_left, new_cursor);
        };
        while new_cursor < tokens.len()
            && matches!(
                tokens[new_cursor].ttype,
                TokenType::Basic(BasicToken::BangEq | BasicToken::EqualEq)
            )
        {
            let operator = Operator::from(&tokens[new_cursor]);
            let (try_right, next_cursor) = self.comparison(tokens, new_cursor + 1);
            let right = if let Ok(right) = try_right {
                right
            } else {
                return (try_right, new_cursor);
            };
            new_cursor = next_cursor;
            left = AstNode::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        (Ok(left), new_cursor)
    }

    fn comparison(&self, tokens: &[TokenItem], cursor: usize) -> (Result<AstNode, Error>, usize) {
        // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
        let (try_left, mut new_cursor) = self.term(tokens, cursor);
        let mut left = if let Ok(left) = try_left {
            left
        } else {
            return (try_left, new_cursor);
        };
        while new_cursor < tokens.len()
            && matches!(
                tokens[new_cursor].ttype,
                TokenType::Basic(
                    BasicToken::Greater
                        | BasicToken::GreaterEq
                        | BasicToken::Less
                        | BasicToken::LessEq
                )
            )
        {
            let operator = Operator::from(&tokens[new_cursor]);
            let (try_right, next_cursor) = self.term(tokens, new_cursor + 1);
            let right = if let Ok(right) = try_right {
                right
            } else {
                return (try_right, new_cursor);
            };
            new_cursor = next_cursor;
            left = AstNode::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        (Ok(left), new_cursor)
    }

    fn term(&self, tokens: &[TokenItem], cursor: usize) -> (Result<AstNode, Error>, usize) {
        // term           → factor ( ( "-" | "+" ) factor )* ;
        let (try_left, mut new_cursor) = self.factor(tokens, cursor);
        let mut left = if let Ok(left) = try_left {
            left
        } else {
            return (try_left, new_cursor);
        };
        while new_cursor < tokens.len()
            && matches!(
                tokens[new_cursor].ttype,
                TokenType::Basic(BasicToken::Minus | BasicToken::Plus)
            )
        {
            let operator = Operator::from(&tokens[new_cursor]);
            let (try_right, next_cursor) = self.factor(tokens, new_cursor + 1);
            let right = if let Ok(right) = try_right {
                right
            } else {
                return (try_right, new_cursor);
            };
            new_cursor = next_cursor;
            left = AstNode::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        (Ok(left), new_cursor)
    }

    fn factor(&self, tokens: &[TokenItem], cursor: usize) -> (Result<AstNode, Error>, usize) {
        // factor         → unary ( ( "/" | "*" ) unary )* ;
        let (try_left, mut new_cursor) = self.unary(tokens, cursor);
        let mut left = if let Ok(left) = try_left {
            left
        } else {
            return (try_left, new_cursor);
        };
        while new_cursor < tokens.len()
            && matches!(
                tokens[new_cursor].ttype,
                TokenType::Basic(BasicToken::Slash | BasicToken::Star)
            )
        {
            let operator = Operator::from(&tokens[new_cursor]);
            let (try_right, next_cursor) = self.unary(tokens, new_cursor + 1);
            let right = if let Ok(right) = try_right {
                right
            } else {
                return (try_right, new_cursor);
            };
            new_cursor = next_cursor;
            left = AstNode::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        (Ok(left), new_cursor)
    }

    fn unary(&self, tokens: &[TokenItem], cursor: usize) -> (Result<AstNode, Error>, usize) {
        // unary          → ( "!" | "-" ) unary | primary ;
        if matches!(
            tokens[cursor].ttype,
            TokenType::Basic(BasicToken::Bang | BasicToken::Minus)
        ) {
            let operator = Operator::from(&tokens[cursor]);
            let (try_right, next_cursor) = self.unary(tokens, cursor + 1);
            let right = if let Ok(right) = try_right {
                right
            } else {
                return (try_right, next_cursor);
            };
            (
                Ok(AstNode::Unary {
                    operator,
                    right: Box::new(right),
                }),
                next_cursor,
            )
        } else {
            self.primary(tokens, cursor)
        }
    }

    fn primary(&self, tokens: &[TokenItem], cursor: usize) -> (Result<AstNode, Error>, usize) {
        // primary        → "true" | "false" | "nil"
        //                | NUMBER | STRING | "(" expression ")"
        match tokens[cursor].ttype {
            TokenType::Literal(LiteralToken::Number | LiteralToken::String)
            | TokenType::Keyword(KeywordToken::True | KeywordToken::False | KeywordToken::Nil) => {
                let value = tokens[cursor]
                    .literal
                    .clone()
                    .expect("Literal token should have a value");
                (Ok(AstNode::Literal { value }), cursor + 1)
            }
            TokenType::Basic(BasicToken::LeftParen) => {
                let (try_expression, next_cursor) = self.equality(tokens, cursor + 1);
                let expression = if let Ok(expression) = try_expression {
                    expression
                } else {
                    return (try_expression, next_cursor);
                };
                if next_cursor < tokens.len()
                    && matches!(
                        tokens[next_cursor].ttype,
                        TokenType::Basic(BasicToken::RightParen)
                    )
                {
                    (Ok(expression), next_cursor + 1)
                } else {
                    (
                        Err(Error::UnterminatedParen {
                            location: tokens[cursor].location,
                        }),
                        cursor,
                    )
                }
            }
            _ => (
                Err(Error::UnexpectedToken {
                    lexeme: tokens[cursor].lexeme.clone(),
                    location: tokens[cursor].location,
                }),
                cursor,
            ),
        }
    }
}
