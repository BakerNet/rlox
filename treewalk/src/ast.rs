use std::{
    cell::RefCell,
    fmt::{Debug, Error},
    rc::Rc,
};

use crate::{
    environment::Environment,
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

type NativeFun =
    Box<dyn Fn(&Vec<&'static str>, Rc<RefCell<Environment>>) -> Result<Literal, Error>>;

pub(crate) struct BuiltinFn {
    pub name: &'static str,
    pub fun: NativeFun,
}

impl Debug for BuiltinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtin {}", self.name)
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
    Builtin {
        params: Vec<&'static str>,
        body: BuiltinFn,
    },
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
            Stmt::Builtin { .. } => SourceLocation::new(0, 0),
        }
    }
}

impl PartialEq for Stmt {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
