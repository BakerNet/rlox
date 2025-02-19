use std::{cell::RefCell, collections::HashMap, rc::Rc};

use thiserror::Error;

use crate::token::Literal;

#[derive(Error, Debug)]
pub enum Error {
    #[error(
        "Resolver Error: Tried to access variable {name} but it wasn't defined in the expected depth"
    )]
    ResolverError { name: String },
}

#[derive(Debug)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Option<Literal>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }

    pub fn new_with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            parent: Some(parent),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Option<Literal>) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn get_at(&self, name: &str, depth: usize) -> Result<Option<Literal>, Error> {
        if depth > 0 {
            match &self.parent {
                Some(parent) => parent.borrow().get_at(name, depth - 1),
                None => Err(Error::ResolverError {
                    name: name.to_string(),
                }),
            }
        } else {
            self.get(name).ok_or(Error::ResolverError {
                name: name.to_string(),
            })
        }
    }

    pub fn get(&self, name: &str) -> Option<Option<Literal>> {
        match self.values.get(name) {
            Some(value) => Some(value.clone()),
            None => match &self.parent {
                Some(parent) => {
                    let parent = parent.borrow();
                    parent.get(name)
                }
                None => None,
            },
        }
    }

    pub fn update_at(
        &mut self,
        name: &str,
        value: Literal,
        depth: usize,
    ) -> Result<Literal, Error> {
        if depth > 0 {
            match &self.parent {
                Some(parent) => parent.borrow_mut().update_at(name, value, depth - 1),
                None => Err(Error::ResolverError {
                    name: name.to_string(),
                }),
            }
        } else {
            self.update(name, value).ok_or(Error::ResolverError {
                name: name.to_string(),
            })
        }
    }

    pub fn update(&mut self, name: &str, value: Literal) -> Option<Literal> {
        match self.values.get_mut(name) {
            Some(v) => {
                *v = Some(value.clone());
                Some(value)
            }
            None => match &self.parent {
                Some(parent) => {
                    let mut parent = parent.borrow_mut();
                    parent.update(name, value)
                }
                None => None,
            },
        }
    }
}
