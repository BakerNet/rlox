use std::collections::HashMap;

use crate::{
    ast::{Expr, Stmt},
    location::SourceLocation,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Can't read local variable '{name}' in its own initializer at {location}")]
    AccessInInitializer {
        name: String,
        location: SourceLocation,
    },

    #[error("Access undeclared variable '{name}' at {location}")]
    AccessUndefined {
        name: String,
        location: SourceLocation,
    },

    #[error("Duplicate variable '{name}' found in scope at {location}")]
    DuplicateVariable {
        name: String,
        location: SourceLocation,
    },
}

trait ResolveExpr {
    fn resolve(
        &self,
        scopes: &mut Vec<HashMap<&'static str, bool>>,
        locals: &mut HashMap<SourceLocation, usize>,
    ) -> Result<(), Error>;
}

impl ResolveExpr for Expr {
    fn resolve(
        &self,
        scopes: &mut Vec<HashMap<&'static str, bool>>,
        locals: &mut HashMap<SourceLocation, usize>,
    ) -> Result<(), Error> {
        match self {
            Expr::Binary { left, right, .. } => {
                left.resolve(scopes, locals)?;
                right.resolve(scopes, locals)?;
                Ok(())
            }
            Expr::Unary { right, .. } => right.resolve(scopes, locals),
            Expr::Literal { .. } => Ok(()),
            Expr::Variable { location, name } => {
                assert!(!scopes.is_empty());
                let last = scopes.len() - 1;
                match scopes[last].get(name) {
                    Some(false) => Err(Error::AccessInInitializer {
                        name: name.to_string(),
                        location: *location,
                    }),
                    _ => {
                        let depth = scopes.iter().rev().enumerate().find_map(|(depth, scope)| {
                            if scope.contains_key(name) {
                                Some(depth)
                            } else {
                                None
                            }
                        });
                        if let Some(depth) = depth {
                            locals.insert(*location, depth);
                            Ok(())
                        } else {
                            Err(Error::AccessUndefined {
                                name: name.to_string(),
                                location: *location,
                            })
                        }
                    }
                }
            }
            Expr::Assignment {
                location,
                name,
                value,
            } => {
                value.resolve(scopes, locals)?;
                let depth = scopes.iter().rev().enumerate().find_map(|(depth, scope)| {
                    if scope.contains_key(name) {
                        Some(depth)
                    } else {
                        None
                    }
                });
                if let Some(depth) = depth {
                    locals.insert(*location, depth);
                    Ok(())
                } else {
                    Err(Error::AccessUndefined {
                        name: name.to_string(),
                        location: *location,
                    })
                }
            }
            Expr::Call {
                callee, arguments, ..
            } => {
                callee.resolve(scopes, locals)?;
                for arg in arguments {
                    arg.resolve(scopes, locals)?;
                }
                Ok(())
            }
        }
    }
}

trait ResolveStmt {
    // the bool is whether the statement is a return statement or not
    fn resolve(
        &self,
        scopes: &mut Vec<HashMap<&'static str, bool>>,
        locals: &mut HashMap<SourceLocation, usize>,
    ) -> Result<(), Error>;
}

impl ResolveStmt for Stmt {
    fn resolve(
        &self,
        scopes: &mut Vec<HashMap<&'static str, bool>>,
        locals: &mut HashMap<SourceLocation, usize>,
    ) -> Result<(), Error> {
        match self {
            Stmt::Expression(expr) => expr.resolve(scopes, locals),
            Stmt::Print(expr) => expr.resolve(scopes, locals),
            Stmt::VarDecl {
                name,
                initializer,
                location,
            } => {
                // If scopes is ever empty, there was an error in the parser
                assert!(!scopes.is_empty());
                let last = scopes.len() - 1;
                if scopes[last].contains_key(name) {
                    return Err(Error::DuplicateVariable {
                        name: name.to_string(),
                        location: *location,
                    });
                }
                scopes[last].insert(name, false);
                if let Some(initializer) = initializer {
                    initializer.resolve(scopes, locals)?;
                }
                scopes[last].insert(name, true);
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                condition.resolve(scopes, locals)?;
                then_branch.resolve(scopes, locals)?;
                if let Some(else_branch) = else_branch {
                    else_branch.resolve(scopes, locals)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                condition.resolve(scopes, locals)?;
                body.resolve(scopes, locals)
            }
            Stmt::Block(vec) => {
                scopes.push(HashMap::new());
                for stmt in vec {
                    stmt.resolve(scopes, locals)?;
                }
                scopes.pop();
                Ok(())
            }
            Stmt::FunDecl { name, params, body } => {
                assert!(!scopes.is_empty());
                let mut last = scopes.len() - 1;
                {
                    scopes[last].insert(name, true);
                }
                // closure
                scopes.push(HashMap::new());
                last += 1;
                for param in params {
                    scopes[last].insert(param, true);
                }
                body.resolve(scopes, locals)?;
                scopes.pop();
                Ok(())
            }
            Stmt::Return(val) => val.resolve(scopes, locals),
            Stmt::Builtin { .. } => Ok(()),
        }
    }
}

pub struct Resolver {}

impl Resolver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn resolve(&self, stmts: &Vec<Stmt>) -> HashMap<SourceLocation, usize> {
        let mut res = HashMap::new();
        let mut scopes = vec![HashMap::new()];
        self.builtin_clock(&mut scopes);
        for stmt in stmts {
            let res = stmt.resolve(&mut scopes, &mut res);
            if let Err(e) = res {
                println!("Resolver Error: {e}");
            }
        }
        res
    }

    fn builtin_clock(&self, scopes: &mut [HashMap<&'static str, bool>]) {
        let last = scopes.len() - 1;
        scopes[last].insert("clock", true);
    }
}
