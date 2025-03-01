use std::{fmt::Display, ptr::write};

macro_rules! non_number {
    ($self:ident, $other:ident) => {
        panic!("Add called on non-Number Values {} <> {}", $self, $other)
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn negate(&self) -> Self {
        match self {
            Value::Number(x) => Value::Number(-x),
            Value::Nil => Value::Nil,
            Value::Bool(b) => Value::Bool(!b),
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a + b),
            _ => non_number!(self, other),
        }
    }

    pub fn subtract(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a - b),
            _ => non_number!(self, other),
        }
    }

    pub fn multiply(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a * b),
            _ => non_number!(self, other),
        }
    }

    pub fn divide(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a / b),
            _ => non_number!(self, other),
        }
    }

    pub fn greater(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Bool(a > b),
            _ => non_number!(self, other),
        }
    }

    pub fn less(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Bool(a < b),
            _ => non_number!(self, other),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(false) | Value::Nil => false,
            _ => true,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Nil
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(d) => write!(f, "{d:?}"),
            Value::Nil => write!(f, "Nil"),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

pub(crate) struct ValueVec<'a>(pub &'a Vec<Value>);

impl Display for ValueVec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|v| write!(f, "[{v}]"))
    }
}
