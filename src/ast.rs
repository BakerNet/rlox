use crate::{
    location::SourceLocation,
    token::{Literal, TokenType},
};

#[derive(Debug)]
pub enum Expr<'a> {
    Binary {
        location: SourceLocation,
        left: Box<Expr<'a>>,
        operator: TokenType,
        right: Box<Expr<'a>>,
    },
    Unary {
        location: SourceLocation,
        operator: TokenType,
        right: Box<Expr<'a>>,
    },
    Literal {
        location: SourceLocation,
        value: Literal,
    },
    Variable {
        location: SourceLocation,
        name: &'a str,
    },
    Assignment {
        location: SourceLocation,
        name: &'a str,
        value: Box<Expr<'a>>,
    },
}

#[derive(Debug)]
pub enum Stmt<'a> {
    Expression(Expr<'a>),
    Print(Expr<'a>),
    VarDecl {
        name: &'a str,
        initializer: Option<Expr<'a>>,
    },
    If {
        condition: Expr<'a>,
        then_branch: Box<Stmt<'a>>,
        else_branch: Option<Box<Stmt<'a>>>,
    },
    While {
        condition: Expr<'a>,
        body: Box<Stmt<'a>>,
    },
    Block(Vec<Stmt<'a>>),
}
