use std::rc::Rc;

use crate::{
    ast::{Expr, Stmt},
    location::SourceLocation,
    token::{Literal, TokenItem, TokenType},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Expected ')' after expression at {location}")]
    UnterminatedParen { location: SourceLocation },

    #[error("Expected ';' after expression at {location}")]
    ExpectedSemicolon { location: SourceLocation },

    #[error("Expected '}}' after block at {location}")]
    UnterminatedBrace { location: SourceLocation },

    #[error("Expected '{expected}' at after '{stmt_type}' {location}")]
    ExpectedToken {
        expected: String,
        stmt_type: String,
        location: SourceLocation,
    },

    #[error("Invalid assignment target at {location}")]
    InvalidAssignmentTarget { location: SourceLocation },

    #[error("Unexpected token '{lexeme}'.  Expected expression at {location}")]
    UnexpectedToken {
        lexeme: String,
        location: SourceLocation,
    },

    #[error("Can't have more than 255 arguments at {location}")]
    TooManyArguments { location: SourceLocation },

    #[error("Can't have more than 255 parameters at {location}")]
    TooManyParameters { location: SourceLocation },

    #[error("Expected parameter name at {location}")]
    ExpectedParameterName { location: SourceLocation },
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
            let operator = $tokens[new_cursor].ttype;
            let (try_right, next_cursor) = $self.$next($tokens, new_cursor + 1);
            let right = if let Ok(right) = try_right {
                right
            } else {
                return (try_right, new_cursor);
            };
            new_cursor = next_cursor;
            left = Expr::Binary {
                location: $tokens[new_cursor].location,
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

    pub fn parse(self, source: Vec<TokenItem>) -> Result<Vec<Stmt>, Vec<Error>> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();
        let mut cursor = 0;
        while cursor < source.len() && !matches!(source[cursor].ttype, TokenType::EoF) {
            let (stmt, next_cursor) = self.statement(&source, cursor);
            cursor = next_cursor;
            match stmt {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    errors.push(err);
                    cursor = self.synchronize(&source, cursor + 1);
                }
            }
        }
        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn statement(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Stmt, Error>, usize) {
        match tokens[cursor].ttype {
            TokenType::Print => self.print_stmt(tokens, cursor + 1),
            TokenType::Var => self.var_decl(tokens, cursor + 1),
            TokenType::LeftBrace => self.block(tokens, cursor + 1),
            TokenType::If => self.if_stmt(tokens, cursor + 1),
            TokenType::While => self.while_stmt(tokens, cursor + 1),
            TokenType::For => self.for_stmt(tokens, cursor + 1),
            TokenType::Fun => self.fun_stmt(tokens, cursor + 1),
            TokenType::Return => self.return_stmt(tokens, cursor + 1),
            _ => self.expr_stmt(tokens, cursor),
        }
    }

    fn expr_stmt(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Stmt, Error>, usize) {
        let (expr, cursor) = self.expression(tokens, cursor);
        if tokens[cursor].ttype == TokenType::Semicolon {
            (expr.map(Stmt::Expression), cursor + 1)
        } else {
            (
                Err(Error::ExpectedSemicolon {
                    location: tokens[cursor].location,
                }),
                cursor,
            )
        }
    }

    fn return_stmt(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Stmt, Error>, usize) {
        if tokens[cursor].ttype == TokenType::Semicolon {
            return (
                Ok(Stmt::Return(Expr::Literal {
                    location: tokens[cursor].location,
                    value: Literal::Nil,
                })),
                cursor + 1,
            );
        }
        let (expr, cursor) = self.expression(tokens, cursor);
        if tokens[cursor].ttype == TokenType::Semicolon {
            (expr.map(Stmt::Return), cursor + 1)
        } else {
            (
                Err(Error::ExpectedSemicolon {
                    location: tokens[cursor].location,
                }),
                cursor,
            )
        }
    }

    fn print_stmt(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Stmt, Error>, usize) {
        let (expr, cursor) = self.equality(tokens, cursor);
        if tokens[cursor].ttype == TokenType::Semicolon {
            (expr.map(Stmt::Print), cursor + 1)
        } else {
            (
                Err(Error::ExpectedSemicolon {
                    location: tokens[cursor].location,
                }),
                cursor,
            )
        }
    }

    fn var_decl(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Stmt, Error>, usize) {
        if !matches!(tokens[cursor].ttype, TokenType::Identifier) {
            return (
                Err(Error::UnexpectedToken {
                    lexeme: tokens[cursor].lexeme.to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }
        let name = tokens[cursor].lexeme;
        let cursor = cursor + 1;
        match tokens[cursor].ttype {
            TokenType::Semicolon => (
                Ok(Stmt::VarDecl {
                    name,
                    location: tokens[cursor].location,
                    initializer: None,
                }),
                cursor + 1,
            ),
            TokenType::Equal => {
                let (expr, cursor) = self.equality(tokens, cursor + 1);
                if tokens[cursor].ttype == TokenType::Semicolon {
                    (
                        expr.map(|expr| Stmt::VarDecl {
                            name,
                            location: tokens[cursor].location,
                            initializer: Some(expr),
                        }),
                        cursor + 1,
                    )
                } else {
                    (
                        Err(Error::ExpectedSemicolon {
                            location: tokens[cursor].location,
                        }),
                        cursor,
                    )
                }
            }
            _ => (
                Err(Error::UnexpectedToken {
                    lexeme: tokens[cursor].lexeme.to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            ),
        }
    }

    fn if_stmt(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Stmt, Error>, usize) {
        if !matches!(tokens[cursor].ttype, TokenType::LeftParen) {
            return (
                Err(Error::ExpectedToken {
                    expected: "(".to_string(),
                    stmt_type: "if".to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }
        let (condition, cursor) = self.expression(tokens, cursor + 1);
        let Ok(condition) = condition else {
            return (condition.map(Stmt::Expression), cursor);
        };
        if !matches!(tokens[cursor].ttype, TokenType::RightParen) {
            return (
                Err(Error::ExpectedToken {
                    expected: ")".to_string(),
                    stmt_type: "if".to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }
        let (then_branch, cursor) = self.statement(tokens, cursor + 1);
        let Ok(then_branch) = then_branch else {
            return (then_branch, cursor);
        };
        let else_branch = if matches!(tokens[cursor].ttype, TokenType::Else) {
            let (else_branch, cursor) = self.statement(tokens, cursor + 1);
            let Ok(else_branch) = else_branch else {
                return (else_branch, cursor);
            };
            Some(else_branch)
        } else {
            None
        };
        (
            Ok(Stmt::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: else_branch.map(Box::new),
            }),
            cursor,
        )
    }

    fn while_stmt(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Stmt, Error>, usize) {
        if !matches!(tokens[cursor].ttype, TokenType::LeftParen) {
            return (
                Err(Error::ExpectedToken {
                    expected: "(".to_string(),
                    stmt_type: "while".to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }
        let (condition, cursor) = self.expression(tokens, cursor + 1);
        let Ok(condition) = condition else {
            return (condition.map(Stmt::Expression), cursor);
        };
        if !matches!(tokens[cursor].ttype, TokenType::RightParen) {
            return (
                Err(Error::ExpectedToken {
                    expected: ")".to_string(),
                    stmt_type: "while".to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }
        if !matches!(tokens[cursor].ttype, TokenType::RightParen) {
            return (
                Err(Error::ExpectedToken {
                    expected: ")".to_string(),
                    stmt_type: "while".to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }
        let (block, cursor) = self.statement(tokens, cursor + 1);
        let Ok(block) = block else {
            return (block, cursor);
        };
        (
            Ok(Stmt::While {
                condition,
                body: Box::new(block),
            }),
            cursor,
        )
    }

    fn for_stmt(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Stmt, Error>, usize) {
        if !matches!(tokens[cursor].ttype, TokenType::LeftParen) {
            return (
                Err(Error::ExpectedToken {
                    expected: "(".to_string(),
                    stmt_type: "for".to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }
        let cursor = cursor + 1;
        let (initializer, cursor) = match tokens[cursor].ttype {
            TokenType::Semicolon => (None, cursor + 1),
            TokenType::Var => {
                let (var_decl, cursor) = self.var_decl(tokens, cursor + 1);
                let Ok(var_decl) = var_decl else {
                    return (var_decl, cursor);
                };
                (Some(var_decl), cursor)
            }
            _ => {
                let (expr_stmt, cursor) = self.expr_stmt(tokens, cursor + 1);
                let Ok(expr_stmt) = expr_stmt else {
                    return (expr_stmt, cursor);
                };
                (Some(expr_stmt), cursor)
            }
        };
        let (condition, cursor) = match tokens[cursor].ttype {
            TokenType::Semicolon => (None, cursor + 1),
            _ => {
                let (condition, cursor) = self.expression(tokens, cursor);
                let Ok(condition) = condition else {
                    return (condition.map(Stmt::Expression), cursor);
                };
                if !matches!(tokens[cursor].ttype, TokenType::Semicolon) {
                    return (
                        Err(Error::ExpectedSemicolon {
                            location: tokens[cursor].location,
                        }),
                        cursor,
                    );
                }
                (Some(condition), cursor + 1)
            }
        };
        let condition = condition.unwrap_or(Expr::Literal {
            location: tokens[cursor].location,
            value: Literal::True,
        });
        let (increment, cursor) = match tokens[cursor].ttype {
            TokenType::RightParen => (None, cursor + 1),
            _ => {
                let (expr, cursor) = self.expression(tokens, cursor);
                let expr = expr.map(Stmt::Expression);
                let Ok(expr) = expr else {
                    return (expr, cursor);
                };
                if !matches!(tokens[cursor].ttype, TokenType::RightParen) {
                    return (
                        Err(Error::ExpectedToken {
                            expected: ")".to_string(),
                            stmt_type: "for".to_string(),
                            location: tokens[cursor].location,
                        }),
                        cursor,
                    );
                }
                (Some(expr), cursor + 1)
            }
        };
        let (body, cursor) = self.statement(tokens, cursor);
        let Ok(body) = body else {
            return (body, cursor);
        };
        let body = if increment.is_some() {
            Stmt::Block(vec![body, increment.unwrap()])
        } else {
            body
        };

        let for_loop = if initializer.is_some() {
            Stmt::Block(vec![
                initializer.unwrap(),
                Stmt::While {
                    condition,
                    body: Box::new(body),
                },
            ])
        } else {
            Stmt::While {
                condition,
                body: Box::new(body),
            }
        };

        (Ok(for_loop), cursor)
    }

    fn fun_stmt(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Stmt, Error>, usize) {
        if !matches!(tokens[cursor].ttype, TokenType::Identifier) {
            return (
                Err(Error::UnexpectedToken {
                    lexeme: tokens[cursor].lexeme.to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }
        let name = tokens[cursor].lexeme;
        let mut cursor = cursor + 1;
        if !matches!(tokens[cursor].ttype, TokenType::LeftParen) {
            return (
                Err(Error::UnexpectedToken {
                    lexeme: tokens[cursor].lexeme.to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }
        cursor += 1;
        let mut params = Vec::new();
        if !matches!(tokens[cursor].ttype, TokenType::RightParen) {
            if matches!(tokens[cursor].ttype, TokenType::Identifier) {
                params.push(tokens[cursor].lexeme);
            } else {
                return (
                    Err(Error::ExpectedParameterName {
                        location: tokens[cursor].location,
                    }),
                    cursor,
                );
            }
            cursor += 1;
            while cursor < tokens.len() && matches!(tokens[cursor].ttype, TokenType::Comma) {
                if params.len() >= 255 {
                    return (
                        Err(Error::TooManyParameters {
                            location: tokens[cursor].location,
                        }),
                        cursor,
                    );
                }
                cursor += 1;

                if matches!(tokens[cursor].ttype, TokenType::Identifier) {
                    params.push(tokens[cursor].lexeme);
                } else {
                    return (
                        Err(Error::ExpectedParameterName {
                            location: tokens[cursor].location,
                        }),
                        cursor,
                    );
                }
                cursor += 1;
            }
        }
        if !matches!(tokens[cursor].ttype, TokenType::RightParen) {
            return (
                Err(Error::ExpectedToken {
                    expected: ")".to_string(),
                    stmt_type: "function".to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }
        cursor += 1;
        if !matches!(tokens[cursor].ttype, TokenType::LeftBrace) {
            return (
                Err(Error::ExpectedToken {
                    expected: "{".to_string(),
                    stmt_type: "function".to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }
        let (try_block, cursor) = self.block(tokens, cursor + 1);
        let Ok(block) = try_block else {
            return (try_block, cursor);
        };
        (
            Ok(Stmt::FunDecl {
                name,
                params,
                body: Rc::new(block),
            }),
            cursor,
        )
    }

    fn block(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Stmt, Error>, usize) {
        let mut stmts = Vec::new();

        let mut cursor = cursor;
        while cursor < tokens.len()
            && !matches!(tokens[cursor].ttype, TokenType::RightBrace | TokenType::EoF)
        {
            let (stmt, next_cursor) = self.statement(tokens, cursor);
            cursor = next_cursor;
            let Ok(stmt) = stmt else {
                return (stmt, cursor);
            };
            stmts.push(stmt);
        }

        if !matches!(tokens[cursor].ttype, TokenType::RightBrace,) {
            return (
                Err(Error::UnterminatedBrace {
                    location: tokens[cursor].location,
                }),
                cursor,
            );
        }

        (Ok(Stmt::Block(stmts)), cursor + 1)
    }

    fn expression(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        self.assignment(tokens, cursor)
    }

    fn assignment(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // assignment     → IDENTIFIER "=" assignment | equality ;
        let (expr, cursor) = self.equality(tokens, cursor);
        let Ok(expr) = expr else {
            return (expr, cursor);
        };
        if !matches!(tokens[cursor].ttype, TokenType::Equal) {
            return (Ok(expr), cursor);
        }
        let assignment_location = tokens[cursor].location;
        let (value, cursor) = self.expression(tokens, cursor + 1);
        let Ok(value) = value else {
            return (value, cursor);
        };
        match expr {
            Expr::Variable { name, location } => (
                Ok(Expr::Assignment {
                    location,
                    name,
                    value: Box::new(value),
                }),
                cursor,
            ),
            _ => (
                Err(Error::InvalidAssignmentTarget {
                    location: assignment_location,
                }),
                cursor,
            ),
        }
    }

    fn equality(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
        binary_expr!(
            self,
            tokens,
            cursor,
            comparison,
            TokenType::BangEq | TokenType::EqualEq
        )
    }

    fn comparison(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
        binary_expr!(
            self,
            tokens,
            cursor,
            term,
            TokenType::Greater | TokenType::GreaterEq | TokenType::Less | TokenType::LessEq
        )
    }

    fn term(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // term           → factor ( ( "-" | "+" ) factor )* ;
        binary_expr!(
            self,
            tokens,
            cursor,
            factor,
            TokenType::Minus | TokenType::Plus
        )
    }

    fn factor(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // factor         → unary ( ( "/" | "*" ) unary )* ;
        binary_expr!(
            self,
            tokens,
            cursor,
            unary,
            TokenType::Slash | TokenType::Star
        )
    }

    fn unary(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // unary          → ( "!" | "-" ) unary | primary ;
        if matches!(tokens[cursor].ttype, TokenType::Bang | TokenType::Minus) {
            let operator = tokens[cursor].ttype;
            let (try_right, next_cursor) = self.unary(tokens, cursor + 1);
            let Ok(right) = try_right else {
                return (try_right, next_cursor);
            };
            (
                Ok(Expr::Unary {
                    location: tokens[cursor].location,
                    operator,
                    right: Box::new(right),
                }),
                next_cursor,
            )
        } else {
            self.call(tokens, cursor)
        }
    }

    fn call(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        let (try_callee, next_cursor) = self.primary(tokens, cursor);
        let Ok(mut callee) = try_callee else {
            return (try_callee, next_cursor);
        };
        let mut cursor = next_cursor;
        while cursor < tokens.len() - 1 && matches!(tokens[cursor].ttype, TokenType::LeftParen) {
            let mut arguments = Vec::new();
            cursor += 1;
            if !matches!(tokens[cursor].ttype, TokenType::RightParen) {
                while cursor < tokens.len()
                    && !matches!(tokens[cursor].ttype, TokenType::RightParen | TokenType::EoF)
                {
                    let (try_arg, next_cursor) = self.expression(tokens, cursor);
                    let Ok(arg) = try_arg else {
                        return (try_arg, next_cursor);
                    };
                    arguments.push(arg);
                    cursor = next_cursor;
                    while cursor < tokens.len() && matches!(tokens[cursor].ttype, TokenType::Comma)
                    {
                        if arguments.len() >= 255 {
                            return (
                                Err(Error::TooManyArguments {
                                    location: tokens[cursor].location,
                                }),
                                cursor,
                            );
                        }
                        let (try_arg, next_cursor) = self.expression(tokens, cursor + 1);
                        let Ok(arg) = try_arg else {
                            return (try_arg, next_cursor);
                        };
                        arguments.push(arg);
                        cursor = next_cursor;
                    }
                }
            }
            if !matches!(tokens[cursor].ttype, TokenType::RightParen) {
                return (
                    Err(Error::UnterminatedParen {
                        location: tokens[cursor].location,
                    }),
                    cursor,
                );
            }
            callee = Expr::Call {
                callee: callee.into(),
                location: tokens[cursor].location,
                arguments,
            };
            cursor += 1;
        }
        (Ok(callee), cursor)
    }

    fn primary(&self, tokens: &[TokenItem], cursor: usize) -> (Result<Expr, Error>, usize) {
        // primary        → "true" | "false" | "nil"
        //                | NUMBER | STRING | "(" expression ")"
        match tokens[cursor].ttype {
            TokenType::Number
            | TokenType::String
            | TokenType::True
            | TokenType::False
            | TokenType::Nil => {
                let value = tokens[cursor]
                    .literal
                    .clone()
                    .expect("Literal token should have a value");
                let location = tokens[cursor].location;
                (Ok(Expr::Literal { location, value }), cursor + 1)
            }
            TokenType::Identifier => {
                let name = tokens[cursor].lexeme;
                let location = tokens[cursor].location;
                (Ok(Expr::Variable { location, name }), cursor + 1)
            }
            TokenType::LeftParen => {
                let (try_expression, next_cursor) = self.equality(tokens, cursor + 1);
                let expression = if let Ok(expression) = try_expression {
                    expression
                } else {
                    return (try_expression, next_cursor);
                };
                if matches!(tokens[next_cursor].ttype, TokenType::RightParen) {
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
                    lexeme: tokens[cursor].lexeme.to_string(),
                    location: tokens[cursor].location,
                }),
                cursor,
            ),
        }
    }

    fn synchronize(&self, source: &[TokenItem], cursor: usize) -> usize {
        let mut cursor = cursor;
        while cursor < source.len() {
            match source[cursor].ttype {
                TokenType::Semicolon => return cursor + 1,
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return cursor,
                _ => cursor += 1,
            }
        }
        cursor
    }
}
