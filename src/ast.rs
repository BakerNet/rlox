use crate::{
    location::SourceLocation,
    token::{Literal, TokenType},
};

#[derive(Debug)]
pub enum Expr {
    Binary {
        location: SourceLocation,
        left: Box<Expr>,
        operator: TokenType,
        right: Box<Expr>,
    },
    Unary {
        location: SourceLocation,
        operator: TokenType,
        right: Box<Expr>,
    },
    Literal {
        location: SourceLocation,
        value: Literal,
    },
    Variable {
        location: SourceLocation,
        name: String,
    },
    Assignment {
        location: SourceLocation,
        name: String,
        value: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    VarDecl {
        name: String,
        initializer: Option<Expr>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Block(Vec<Stmt>),
}
