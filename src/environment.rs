use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::token::Literal;

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

    pub fn define(&mut self, name: String, value: Option<Literal>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Option<Literal>> {
        match self.values.get(name) {
            Some(value) => Some(value.clone()),
            None => match &self.parent {
                Some(parent) => {
                    let parent = parent.borrow();
                    let value = parent.get(name);
                    value.clone()
                }
                None => None,
            },
        }
    }

    pub fn update(&mut self, name: String, value: Literal) -> Option<Literal> {
        match self.values.get(&name) {
            Some(_) => {
                self.values.insert(name, Some(value.clone()));
                Some(value)
            }
            None => match &self.parent {
                Some(parent) => {
                    let mut parent = parent.borrow_mut();
                    let value = parent.update(name, value);
                    value.clone()
                }
                None => None,
            },
        }
    }
}
