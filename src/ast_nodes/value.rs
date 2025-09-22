use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Value {
    pub fn as_num(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::String(_) => 0.0,
            Value::Bool(b) => if *b { 1.0 } else { 0.0 },
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Bool(b) => *b,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(n) => n.clone(),
            Value::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(lhs) => {
                match rhs {
                    Value::Number(rhs) => Value::Number(lhs + rhs),
                    Value::Bool(_) => Value::Number(lhs + rhs.as_num()),
                    Value::String(rhs) => Value::String(format!("{}{}", lhs, rhs)),
                }
            },
            Value::Bool(_) => {
                match rhs {
                    Value::Number(rhs) => Value::Number(self.as_num() + rhs),
                    Value::Bool(_) => Value::Number(self.as_num() + rhs.as_num()),
                    Value::String(rhs) => Value::String(format!("{}{}", self.to_string(), rhs)),
                }
            }
            Value::String(lhs) => Value::String(format!("{}{}", lhs, rhs)),
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

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => Value::Number(-n),
            Value::String(_) => Value::String(String::from("Nan")),
            Value::Bool(b) => Value::Number(if b { -1.0 } else { 0.0 })
        }
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        match self {
            Value::Number(n) => if n != 0.0 { Value::Bool(false) } else { Value::Bool(true) },
            Value::String(_) => Value::String(String::from("Nan")),
            Value::Bool(b) => Value::Bool(!b)
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        only_number_with_number_op(self, rhs, |lhs, rhs| lhs / rhs)
    }
}

fn only_number_with_number_op(lhs: Value, rhs: Value, action: fn(f64, f64) -> f64) -> Value {
    match lhs {
        Value::Number(lhs) => {
            match rhs {
                Value::Number(rhs) => Value::Number(action(lhs, rhs)),
                Value::Bool(_) => Value::Number(action(lhs, rhs.as_num())),
                Value::String(_) => Value::String(String::from("Nan")),
            }
        },
        Value::Bool(_) => {
            match rhs {
                Value::Number(rhs_num) => Value::Number(action(lhs.as_num(), rhs_num)),
                Value::Bool(_) => Value::Number(action(lhs.as_num(), rhs.as_num())),
                Value::String(_) => Value::String(String::from("Nan")),
            }
        }
        Value::String(_) => Value::String(String::from("Nan"))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::String(s) => write!(f, "String(\"{}\")", s),
            Value::Bool(b) => write!(f, "Bool(\"{}\")", b),
        }
    }
}