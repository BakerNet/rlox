use std::cmp::Ordering;

use crate::{
    ast::{AstNode, Expr},
    location::SourceLocation,
    token::{BasicToken, Literal, TokenType},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected character `{message}` at {location}")]
    RuntimeError {
        message: String,
        location: SourceLocation,
    },

    #[error("Parser failed to parse expression at {location}")]
    ParseError { location: SourceLocation },
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, ast: &AstNode) -> Result<Literal, Error> {
        match ast {
            AstNode::Expression(expr) => self.evaluate(expr),
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Literal, Error> {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(&*left)?;
                let right = self.evaluate(&*right)?;
                let res = match operator.ttype {
                    TokenType::Basic(BasicToken::EqualEq) => Literal::from(left == right),
                    TokenType::Basic(BasicToken::BangEq) => Literal::from(left != right),
                    TokenType::Basic(BasicToken::Greater) => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: operator.location,
                        })?;
                        Literal::from(matches!(comp, Ordering::Greater))
                    }
                    TokenType::Basic(BasicToken::Less) => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: operator.location,
                        })?;
                        Literal::from(matches!(comp, Ordering::Less))
                    }
                    TokenType::Basic(BasicToken::GreaterEq) => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: operator.location,
                        })?;
                        Literal::from(matches!(comp, Ordering::Greater | Ordering::Equal))
                    }
                    TokenType::Basic(BasicToken::LessEq) => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: operator.location,
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
                                location: operator.location,
                            });
                        }
                    },
                    TokenType::Basic(BasicToken::Minus) => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a - b),
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot subtract values. Operands must be both numbers"
                                    .to_string(),
                                location: operator.location,
                            });
                        }
                    },
                    TokenType::Basic(BasicToken::Star) => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a * b),
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot multiply values. Operands must be both numbers"
                                    .to_string(),
                                location: operator.location,
                            });
                        }
                    },
                    TokenType::Basic(BasicToken::Slash) => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => {
                            if b == 0.0 {
                                return Err(Error::RuntimeError {
                                    message: "Cannot divide by zero".to_string(),
                                    location: operator.location,
                                });
                            }
                            Literal::Number(a / b)
                        }
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot divide values. Operands must be both numbers"
                                    .to_string(),
                                location: operator.location,
                            });
                        }
                    },
                    _ => {
                        return Err(Error::ParseError {
                            location: operator.location,
                        });
                    }
                };
                Ok(res)
            }
            Expr::Unary { operator, right } => {
                let right = self.evaluate(&*right)?;
                let res = match operator.ttype {
                    TokenType::Basic(BasicToken::Minus) => match right {
                        Literal::Number(n) => Literal::Number(-n),
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot negate a non-number".to_string(),
                                location: operator.location,
                            });
                        }
                    },
                    TokenType::Basic(BasicToken::Bang) => Literal::from(!right.is_truthy()),
                    _ => {
                        return Err(Error::ParseError {
                            location: operator.location,
                        });
                    }
                };
                Ok(res)
            }
            Expr::Literal { value } => Ok(value.clone()),
        }
    }
}
