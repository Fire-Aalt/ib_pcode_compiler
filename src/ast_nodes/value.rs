use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Array(usize),
    Instance(String, usize),
}

impl Value {
    pub fn as_num(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::String(_) => 0.0,
            Value::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            _ => panic!("Cannot convert {} to num", self),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Bool(b) => *b,
            _ => panic!("Cannot convert {} to bool", self),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(f) => Value::Number(-f),
            Value::Bool(b) => Value::Number(if b { -1.0 } else { 0.0 }),
            _ => Value::String(String::from("Nan")),
        }
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        match self {
            Value::Number(f) => {
                if f != 0.0 {
                    Value::Bool(false)
                } else {
                    Value::Bool(true)
                }
            }
            Value::Bool(b) => Value::Bool(!b),
            _ => Value::String(String::from("Nan")),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::String(lhs) => Value::String(format!("{}{}", lhs, rhs)),
            _ => match rhs {
                Value::String(rhs) => Value::String(format!("{}{}", self, rhs)),
                _ => Value::Number(self.as_num() + rhs.as_num()),
            },
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        only_number_with_number_op(self, rhs, |lhs, rhs| lhs - rhs)
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        only_number_with_number_op(self, rhs, |lhs, rhs| lhs * rhs)
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        only_number_with_number_op(self, rhs, |lhs, rhs| lhs / rhs)
    }
}

fn only_number_with_number_op(lhs: Value, rhs: Value, op: fn(f64, f64) -> f64) -> Value {
    if let Value::String(_) = rhs {
        return Value::String(String::from("Nan"));
    }

    match lhs {
        Value::Number(_) => Value::Number(op(lhs.as_num(), rhs.as_num())),
        Value::Bool(_) => Value::Number(op(lhs.as_num(), rhs.as_num())),
        _ => Value::String(String::from("Nan")),
    }
}

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(f) => write!(formatter, "Number({})", f),
            Value::String(s) => write!(formatter, "String(\"{}\")", s),
            Value::Bool(b) => write!(formatter, "Bool({})", b),
            Value::Array(id) => write!(formatter, "Array(Id: {})", id),
            Value::Instance(name, id) => write!(formatter, "Instance(Name: {}, Id: {})", name, id),
        }
    }
}
