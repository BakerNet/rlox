use std::{fmt::Display, rc::Rc};

macro_rules! non_number {
    ($op:expr, $self:ident, $other:ident) => {
        panic!(
            "{} called on non-Number Values {} <> {}",
            $op, $self, $other
        )
    };
    ($op:expr, $self:ident) => {
        panic!("{} called on non-Number Value {}", $op, $self)
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Number(f64),
    Bool(bool),
    ConstString(&'a str), // points to source code
    String(Rc<String>),   // Rc instead of Garbage collector
    Nil,
}

impl Value<'_> {
    pub fn negate(&self) -> Self {
        match self {
            Value::Number(x) => Value::Number(-x),
            _ => non_number!("Negate", self),
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a + b),
            (Self::String(a), b) => Self::String(Rc::new(format!("{}{}", a, b))),
            (a, Self::String(b)) => Self::String(Rc::new(format!("{}{}", a, b))),
            (Self::ConstString(a), b) => Self::String(Rc::new(format!("{}{}", a, b))),
            (a, Self::ConstString(b)) => Self::String(Rc::new(format!("{}{}", a, b))),
            _ => non_number!("Add", self, other),
        }
    }

    pub fn subtract(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a - b),
            _ => non_number!("Subtract", self, other),
        }
    }

    pub fn multiply(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a * b),
            _ => non_number!("Multiply", self, other),
        }
    }

    pub fn divide(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a / b),
            _ => non_number!("Divide", self, other),
        }
    }

    pub fn greater(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Bool(a > b),
            _ => non_number!("Greater", self, other),
        }
    }

    pub fn less(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Bool(a < b),
            _ => non_number!("Less", self, other),
        }
    }

    pub fn is_truthy(&self) -> bool {
        matches!(self, Value::Bool(false) | Value::Nil)
    }
}

impl Default for Value<'_> {
    fn default() -> Self {
        Self::Nil
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(d) => write!(f, "{d:?}"),
            Value::Nil => write!(f, "Nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::ConstString(s) => write!(f, "{}", *s),
            Value::String(s) => write!(f, "{}", *s),
        }
    }
}

pub(crate) struct ValueVec<'a>(pub &'a Vec<Value<'a>>);

impl Display for ValueVec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|v| write!(f, "[{v}]"))
    }
}
