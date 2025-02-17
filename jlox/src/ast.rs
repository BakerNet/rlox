use std::rc::Rc;

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
    Call {
        location: SourceLocation,
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Literal {
        location: SourceLocation,
        value: Literal,
    },
    Variable {
        location: SourceLocation,
        name: &'static str,
    },
    Assignment {
        location: SourceLocation,
        name: &'static str,
        value: Box<Expr>,
    },
}

impl Expr {
    pub(crate) fn location(&self) -> SourceLocation {
        match self {
            Expr::Binary { location, .. } => *location,
            Expr::Unary { location, .. } => *location,
            Expr::Call { location, .. } => *location,
            Expr::Literal { location, .. } => *location,
            Expr::Variable { location, .. } => *location,
            Expr::Assignment { location, .. } => *location,
        }
    }
}

// TODO - these should all have location. Using location of Exprs is misleading
#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    VarDecl {
        name: &'static str,
        location: SourceLocation,
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
    FunDecl {
        name: &'static str,
        params: Vec<&'static str>,
        body: Rc<Stmt>,
    },
    Return(Expr),
}

impl Stmt {
    pub(crate) fn location(&self) -> SourceLocation {
        match self {
            Stmt::Expression(expr) => expr.location(),
            Stmt::Print(expr) => expr.location(),
            Stmt::VarDecl { location, .. } => *location,
            Stmt::If { condition, .. } => condition.location(),
            Stmt::While { condition, .. } => condition.location(),
            Stmt::Block(stmts) => {
                if !stmts.is_empty() {
                    stmts[0].location()
                } else {
                    SourceLocation::new(0, 0)
                }
            }
            Stmt::FunDecl { body, .. } => body.location(),
            Stmt::Return(expr) => expr.location(),
        }
    }
}

impl PartialEq for Stmt {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
