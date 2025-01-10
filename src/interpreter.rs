use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use crate::{
    ast::{Expr, Stmt},
    environment::Environment,
    location::SourceLocation,
    token::{BasicToken, Literal, TokenType},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Runtime Error: {message} at {location}")]
    RuntimeError {
        message: String,
        location: SourceLocation,
    },

    #[error("Parser failed to parse expression at {location}")]
    ParseError { location: SourceLocation },
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&self, stmts: Vec<Stmt>) -> Result<Option<Literal>, Error> {
        let mut res = None;
        for stmt in stmts {
            res = self.execute(&stmt, self.environment.clone())?;
        }
        Ok(res)
    }

    fn execute(
        &self,
        stmt: &Stmt,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Option<Literal>, Error> {
        match stmt {
            Stmt::Expression(expr) => {
                let value = self.evaluate(expr, environment)?;
                Ok(Some(value))
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr, environment)?;
                println!("{}", value);
                Ok(None)
            }
            Stmt::VarDecl { name, initializer } => {
                let value = match initializer {
                    Some(expr) => Some(self.evaluate(expr, environment.clone())?),
                    None => None,
                };
                environment.borrow_mut().define(name.clone(), value);
                Ok(None)
            }
            Stmt::Block(vec) => {
                let mut res = None;
                let new_env = Rc::new(RefCell::new(Environment::new_with_parent(
                    environment.clone(),
                )));
                for inner in vec {
                    res = self.execute(inner, new_env.clone())?;
                }
                Ok(res)
            }
        }
    }

    fn evaluate(
        &self,
        expr: &Expr,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Literal, Error> {
        match expr {
            Expr::Binary {
                location,
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(&*left, environment.clone())?;
                let right = self.evaluate(&*right, environment.clone())?;
                let res = match operator {
                    TokenType::Basic(BasicToken::EqualEq) => Literal::from(left == right),
                    TokenType::Basic(BasicToken::BangEq) => Literal::from(left != right),
                    TokenType::Basic(BasicToken::Greater) => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: *location,
                        })?;
                        Literal::from(matches!(comp, Ordering::Greater))
                    }
                    TokenType::Basic(BasicToken::Less) => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: *location,
                        })?;
                        Literal::from(matches!(comp, Ordering::Less))
                    }
                    TokenType::Basic(BasicToken::GreaterEq) => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: *location,
                        })?;
                        Literal::from(matches!(comp, Ordering::Greater | Ordering::Equal))
                    }
                    TokenType::Basic(BasicToken::LessEq) => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: *location,
                        })?;
                        Literal::from(matches!(comp, Ordering::Less | Ordering::Equal))
                    }
                    TokenType::Basic(BasicToken::Plus) => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a + b),
                        (Literal::String(a), Literal::String(b)) => {
                            Literal::String(format!("{}{}", a, b))
                        }
                        (Literal::String(a), b) => Literal::String(format!("{}{}", a, b)),
                        (a, Literal::String(b)) => Literal::String(format!("{}{}", a, b)),
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot add values.  Operands must be both numbers or both strings".to_string(),
                                location: *location,
                            });
                        }
                    },
                    TokenType::Basic(BasicToken::Minus) => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a - b),
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot subtract values. Operands must be both numbers"
                                    .to_string(),
                                location: *location,
                            });
                        }
                    },
                    TokenType::Basic(BasicToken::Star) => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a * b),
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot multiply values. Operands must be both numbers"
                                    .to_string(),
                                location: *location,
                            });
                        }
                    },
                    TokenType::Basic(BasicToken::Slash) => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => {
                            if b == 0.0 {
                                return Err(Error::RuntimeError {
                                    message: "Cannot divide by zero".to_string(),
                                    location: *location,
                                });
                            }
                            Literal::Number(a / b)
                        }
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot divide values. Operands must be both numbers"
                                    .to_string(),
                                location: *location,
                            });
                        }
                    },
                    _ => {
                        return Err(Error::ParseError {
                            location: *location,
                        });
                    }
                };
                Ok(res)
            }
            Expr::Unary {
                location,
                operator,
                right,
            } => {
                let right = self.evaluate(&*right, environment.clone())?;
                let res = match operator {
                    TokenType::Basic(BasicToken::Minus) => match right {
                        Literal::Number(n) => Literal::Number(-n),
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot negate a non-number".to_string(),
                                location: *location,
                            });
                        }
                    },
                    TokenType::Basic(BasicToken::Bang) => Literal::from(!right.is_truthy()),
                    _ => {
                        return Err(Error::ParseError {
                            location: *location,
                        });
                    }
                };
                Ok(res)
            }
            Expr::Literal { value, .. } => Ok(value.clone()),
            Expr::Variable { location, name } => environment
                .borrow()
                .get(name)
                .ok_or(Error::RuntimeError {
                    message: format!("Undefined variable `{}`", name),
                    location: *location,
                })?
                .ok_or(Error::RuntimeError {
                    message: format!("Uninitialized variable `{}` used", name),
                    location: *location,
                }),
            Expr::Assignment {
                location,
                name,
                value,
            } => {
                let value = self.evaluate(&*value, environment.clone())?;
                environment
                    .borrow_mut()
                    .update(name.to_string(), value)
                    .ok_or(Error::RuntimeError {
                        message: format!("Undefined variable `{}`", name),
                        location: *location,
                    })
            }
        }
    }
}
