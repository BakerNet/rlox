use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Nil,
}

impl Value {
    pub fn negate(&self) -> Self {
        match self {
            Value::Number(x) => Value::Number(-x),
            Value::Nil => Value::Nil,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a + b),
            _ => Self::Nil,
        }
    }

    pub fn subtract(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a - b),
            _ => Self::Nil,
        }
    }

    pub fn multiply(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a * b),
            _ => Self::Nil,
        }
    }

    pub fn divide(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Self::Number(a / b),
            _ => Self::Nil,
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
        }
    }
}

pub(crate) struct ValueVec<'a>(pub &'a Vec<Value>);

impl Display for ValueVec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|v| write!(f, "[{v}]"))
    }
}
