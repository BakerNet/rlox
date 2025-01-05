use crate::{
    location::SourceLocation,
    token::{Literal, TokenItem, TokenType},
};

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
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    Unary {
        operator: Operator,
        right: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
}

#[derive(Debug)]
pub enum AstNode {
    Expression(Expr),
}
