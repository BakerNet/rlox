use std::{cell::RefCell, cmp::Ordering, collections::HashMap, rc::Rc};

use crate::{
    ast::{Expr, Stmt},
    environment::Environment,
    location::SourceLocation,
    token::{Literal, TokenType},
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

#[derive(Clone, Copy)]
enum FunctionType {
    Function,
    None,
}

trait EvaluateExpr {
    fn evaluate(
        &self,
        environment: Rc<RefCell<Environment>>,
        locals: &HashMap<SourceLocation, usize>,
        function_stack: &mut Vec<FunctionType>,
    ) -> Result<Literal, Error>;
}

impl EvaluateExpr for Expr {
    fn evaluate(
        &self,
        environment: Rc<RefCell<Environment>>,
        locals: &HashMap<SourceLocation, usize>,
        function_stack: &mut Vec<FunctionType>,
    ) -> Result<Literal, Error> {
        match self {
            Expr::Binary {
                location,
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(environment.clone(), locals, function_stack)?;
                let right = right.evaluate(environment, locals, function_stack)?;
                let res = match operator {
                    TokenType::EqualEq => Literal::from(left == right),
                    TokenType::BangEq => Literal::from(left != right),
                    TokenType::Greater => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: *location,
                        })?;
                        Literal::from(matches!(comp, Ordering::Greater))
                    }
                    TokenType::Less => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: *location,
                        })?;
                        Literal::from(matches!(comp, Ordering::Less))
                    }
                    TokenType::GreaterEq => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: *location,
                        })?;
                        Literal::from(matches!(comp, Ordering::Greater | Ordering::Equal))
                    }
                    TokenType::LessEq => {
                        let comp = left.partial_cmp(&right).ok_or(Error::RuntimeError {
                            message: "Cannot compare values. Operands must both be numbers"
                                .to_string(),
                            location: *location,
                        })?;
                        Literal::from(matches!(comp, Ordering::Less | Ordering::Equal))
                    }
                    TokenType::Plus => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a + b),
                        (Literal::String(a), Literal::String(b)) => {
                            Literal::String(format!("{}{}", a, b).into())
                        }
                        (Literal::String(a), b) => Literal::String(format!("{}{}", a, b).into()),
                        (a, Literal::String(b)) => Literal::String(format!("{}{}", a, b).into()),
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot add values.  Operands must be both numbers or both strings".to_string(),
                                location: *location,
                            });
                        }
                    },
                    TokenType::Minus => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a - b),
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot subtract values. Operands must be both numbers"
                                    .to_string(),
                                location: *location,
                            });
                        }
                    },
                    TokenType::Star => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a * b),
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot multiply values. Operands must be both numbers"
                                    .to_string(),
                                location: *location,
                            });
                        }
                    },
                    TokenType::Slash => match (left, right) {
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
                    TokenType::Or => {
                        if left.is_truthy() {
                            return Ok(left);
                        }
                        return Ok(right);
                    }
                    TokenType::And => {
                        if !left.is_truthy() {
                            return Ok(left);
                        }
                        return Ok(right);
                    }
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
                let right = right.evaluate(environment, locals, function_stack)?;
                let res = match operator {
                    TokenType::Minus => match right {
                        Literal::Number(n) => Literal::Number(-n),
                        _ => {
                            return Err(Error::RuntimeError {
                                message: "Cannot negate a non-number".to_string(),
                                location: *location,
                            });
                        }
                    },
                    TokenType::Bang => Literal::from(!right.is_truthy()),
                    _ => {
                        return Err(Error::ParseError {
                            location: *location,
                        });
                    }
                };
                Ok(res)
            }
            Expr::Literal { value, .. } => Ok(value.clone()),
            Expr::Variable { location, name } => {
                let depth = locals.get(location);
                let val =
                    match depth {
                        Some(d) => environment.borrow().get_at(name, *d).map_err(|e| {
                            Error::RuntimeError {
                                message: e.to_string(),
                                location: *location,
                            }
                        })?,
                        None => environment.borrow().get(name).ok_or(Error::RuntimeError {
                            message: format!("Undefined variable `{}`", name),
                            location: *location,
                        })?,
                    };
                val.ok_or(Error::RuntimeError {
                    message: format!("Uninitialized variable `{}` used", name),
                    location: *location,
                })
            }
            Expr::Assignment {
                location,
                name,
                value,
            } => {
                let value = value.evaluate(environment.clone(), locals, function_stack)?;
                let depth = locals.get(location);
                match depth {
                    Some(d) => environment
                        .borrow_mut()
                        .update_at(name, value, *d)
                        .map_err(|e| Error::RuntimeError {
                            message: e.to_string(),
                            location: *location,
                        }),
                    None => {
                        environment
                            .borrow_mut()
                            .update(name, value)
                            .ok_or(Error::RuntimeError {
                                message: format!("Undefined variable `{}`", name),
                                location: *location,
                            })
                    }
                }
            }
            Expr::Call {
                location,
                callee,
                arguments,
            } => {
                let callee = callee.evaluate(environment.clone(), locals, function_stack)?;
                let Literal::Function {
                    params,
                    body,
                    closure,
                } = callee
                else {
                    return Err(Error::RuntimeError {
                        message: "Can only call functions and classes.".to_string(),
                        location: *location,
                    });
                };
                if arguments.len() != params.len() {
                    return Err(Error::RuntimeError {
                        message: format!(
                            "Expected {} arguments bug got {}",
                            params.len(),
                            arguments.len()
                        ),
                        location: *location,
                    });
                }
                let arguments: Result<Vec<Literal>, Error> = arguments
                    .iter()
                    .map(|e| e.evaluate(environment.clone(), locals, function_stack))
                    .collect();
                let Ok(arguments) = arguments else {
                    return Err(arguments.unwrap_err());
                };
                let new_env = Rc::new(RefCell::new(Environment::new_with_parent(closure)));
                params.into_iter().zip(arguments).for_each(|(p, l)| {
                    new_env.borrow_mut().define(p, Some(l));
                });
                function_stack.push(FunctionType::Function);
                let res = body
                    .execute(new_env.clone(), locals, function_stack)
                    .map(|(v, _)| v.unwrap_or(Literal::Nil))?;
                function_stack.pop();
                Ok(res)
            }
        }
    }
}

trait ExecuteStmt {
    // the bool is whether the statement is a return statement or not
    fn execute(
        &self,
        environment: Rc<RefCell<Environment>>,
        locals: &HashMap<SourceLocation, usize>,
        function_stack: &mut Vec<FunctionType>,
    ) -> Result<(Option<Literal>, bool), Error>;
}

impl ExecuteStmt for Stmt {
    fn execute(
        &self,
        environment: Rc<RefCell<Environment>>,
        locals: &HashMap<SourceLocation, usize>,
        function_stack: &mut Vec<FunctionType>,
    ) -> Result<(Option<Literal>, bool), Error> {
        match self {
            Stmt::Expression(expr) => {
                let value = expr.evaluate(environment, locals, function_stack)?;
                Ok((Some(value), false))
            }
            Stmt::Print(expr) => {
                let value = expr.evaluate(environment, locals, function_stack)?;
                println!("{}", value);
                Ok((None, false))
            }
            Stmt::VarDecl {
                name, initializer, ..
            } => {
                let value = match initializer {
                    Some(expr) => {
                        Some(expr.evaluate(environment.clone(), locals, function_stack)?)
                    }
                    None => None,
                };
                environment.borrow_mut().define(name, value);
                Ok((None, false))
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if condition
                    .evaluate(environment.clone(), locals, function_stack)?
                    .is_truthy()
                {
                    then_branch.execute(environment.clone(), locals, function_stack)
                } else if let Some(else_branch) = else_branch {
                    else_branch.execute(environment.clone(), locals, function_stack)
                } else {
                    Ok((None, false))
                }
            }
            Stmt::While { condition, body } => {
                while condition
                    .evaluate(environment.clone(), locals, function_stack)?
                    .is_truthy()
                {
                    let res = body.execute(environment.clone(), locals, function_stack)?;
                    if res.1 {
                        // is return
                        return Ok(res);
                    }
                }
                Ok((None, false))
            }
            Stmt::Block(vec) => {
                let mut res: (Option<Literal>, bool) = (None, false);
                let new_env = Rc::new(RefCell::new(Environment::new_with_parent(
                    environment.clone(),
                )));
                for inner in vec {
                    res = inner.execute(new_env.clone(), locals, function_stack)?;
                    if res.1 {
                        // is return
                        break;
                    }
                }
                if res.1 { Ok(res) } else { Ok((None, false)) }
            }
            Stmt::FunDecl { name, params, body } => {
                let closure = environment.clone();
                environment.borrow_mut().define(
                    name,
                    Some(Literal::Function {
                        params: params.to_vec(),
                        body: body.clone(),
                        closure,
                    }),
                );
                Ok((None, false))
            }
            Stmt::Return(val) => {
                let last = function_stack.len() - 1;
                if matches!(function_stack[last], FunctionType::None) {
                    return Err(Error::RuntimeError {
                        message: "Can't return from outside a function".to_string(),
                        location: val.location(),
                    });
                }
                val.evaluate(environment, locals, function_stack)
                    .map(|l| (Some(l), true))
            }
        }
    }
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<SourceLocation, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
            locals: HashMap::new(),
        }
    }

    pub fn new_with_locals(locals: HashMap<SourceLocation, usize>) -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
            locals,
        }
    }

    pub fn interpret(&self, stmts: Vec<Stmt>) -> Result<Option<Literal>, Error> {
        let mut res = None;
        for stmt in stmts {
            res = stmt
                .execute(
                    self.environment.clone(),
                    &self.locals,
                    &mut vec![FunctionType::None],
                )?
                .0;
        }
        Ok(res)
    }
}
