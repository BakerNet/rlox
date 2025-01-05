use crate::{
    ast::{AstNode, Expr, Operator},
    location::SourceLocation,
    token::{BasicToken, KeywordToken, LiteralToken, TokenItem, TokenType},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Expected ')' after expression at {location}")]
    UnterminatedParen { location: SourceLocation },

    #[error("Unexpected token '{lexeme}'.  Expected expression at {location}")]
    UnexpectedToken {
        lexeme: String,
        location: SourceLocation,
    },
}

macro_rules! binary_expr {
    ($self:ident, $tokens:ident, $cursor:ident, $next:ident, $pattern:pat) => {{
        let (try_left, mut new_cursor) = $self.$next($tokens, $cursor);
        let mut left = if let Ok(left) = try_left {
            left
        } else {
            return (try_left, new_cursor);
        };
        while matches!($tokens[new_cursor].ttype, $pattern) {
            let operator = Operator::from(&$tokens[new_cursor]);
            let (try_right, next_cursor) = $self.$next($tokens, new_cursor + 1);
            let right = if let Ok(right) = try_right {
                right
            } else {
                return (try_right, new_cursor);
            };
            new_cursor = next_cursor;
            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        (Ok(left), new_cursor)
    }};
}

// For chapter 6, we will only parse equality expressions.
pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(self, source: Vec<TokenItem>) -> Result<AstNode, Vec<Error>> {
        let (res, _cursor) = self.expression(&source[..source.len()], 0);
        res.map(|expr| AstNode::Expression(expr))
            .map_err(|e| vec![e])
    }

    fn expression(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        self.equality(tokens, cursor)
    }

    fn equality(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
        binary_expr!(
            self,
            tokens,
            cursor,
            comparison,
            TokenType::Basic(BasicToken::BangEq | BasicToken::EqualEq)
        )
    }

    fn comparison(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
        binary_expr!(
            self,
            tokens,
            cursor,
            term,
            TokenType::Basic(
                BasicToken::Greater | BasicToken::GreaterEq | BasicToken::Less | BasicToken::LessEq
            )
        )
    }

    fn term(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // term           → factor ( ( "-" | "+" ) factor )* ;
        binary_expr!(
            self,
            tokens,
            cursor,
            factor,
            TokenType::Basic(BasicToken::Minus | BasicToken::Plus)
        )
    }

    fn factor(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // factor         → unary ( ( "/" | "*" ) unary )* ;
        binary_expr!(
            self,
            tokens,
            cursor,
            unary,
            TokenType::Basic(BasicToken::Slash | BasicToken::Star)
        )
    }

    fn unary(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
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
                Ok(Expr::Unary {
                    operator,
                    right: Box::new(right),
                }),
                next_cursor,
            )
        } else {
            self.primary(tokens, cursor)
        }
    }

    fn primary(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // primary        → "true" | "false" | "nil"
        //                | NUMBER | STRING | "(" expression ")"
        match tokens[cursor].ttype {
            TokenType::Literal(LiteralToken::Number | LiteralToken::String)
            | TokenType::Keyword(KeywordToken::True | KeywordToken::False | KeywordToken::Nil) => {
                let value = tokens[cursor]
                    .literal
                    .clone()
                    .expect("Literal token should have a value");
                (Ok(Expr::Literal { value }), cursor + 1)
            }
            TokenType::Basic(BasicToken::LeftParen) => {
                let (try_expression, next_cursor) = self.equality(tokens, cursor + 1);
                let expression = if let Ok(expression) = try_expression {
                    expression
                } else {
                    return (try_expression, next_cursor);
                };
                if matches!(
                    tokens[next_cursor].ttype,
                    TokenType::Basic(BasicToken::RightParen)
                ) {
                    (Ok(expression), next_cursor + 1)
                } else {
                    (
                        Err(Error::UnterminatedParen {
                            location: tokens[cursor].location,
                        }),
                        next_cursor,
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
