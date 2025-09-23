use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone)]
pub enum Value {
    Float(f64),
    Int(i64),
    Bool(bool),
    String(String),
}

impl Value {
    pub fn as_num(&self) -> f64 {
        match self {
            Value::Float(n) => *n,
            Value::Int(n) => *n as f64,
            Value::String(_) => 0.0,
            Value::Bool(b) => if *b { 1.0 } else { 0.0 },
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Float(n) => *n != 0.0,
            Value::Int(n) => *n != 0,
            Value::String(s) => !s.is_empty(),
            Value::Bool(b) => *b,
        }
    }
}



impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::String(lhs) => Value::String(format!("{}{}", lhs, rhs)),
            _ => {
                match rhs {
                    Value::String(rhs) => Value::String(format!("{}{}", self, rhs)),
                    _ => number_op_both_int(self, rhs, |lhs, rhs | lhs + rhs, |lhs, rhs | lhs.checked_add(rhs))
                }
            }
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        only_number_with_number_op(self, rhs, |lhs, rhs| lhs - rhs, |lhs, rhs| lhs.checked_sub(rhs), number_op_both_int)
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        only_number_with_number_op(self, rhs, |lhs, rhs| lhs * rhs, |lhs, rhs| lhs.checked_mul(rhs), number_op_both_int)
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::Float(f) => Value::Float(-f),
            Value::Int(i) => Value::Int(-i),
            Value::Bool(b) => Value::Int(if b { -1 } else { 0 }),
            Value::String(_) => Value::String(String::from("Nan")),
        }
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        match self {
            Value::Float(f) => if f != 0.0 { Value::Bool(false) } else { Value::Bool(true) },
            Value::Int(i) => if i != 0 { Value::Bool(false) } else { Value::Bool(true) },
            Value::Bool(b) => Value::Bool(!b),
            Value::String(_) => Value::String(String::from("Nan")),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        only_number_with_number_op(self, rhs, |lhs, rhs| lhs / rhs, |lhs, rhs| lhs.checked_div(rhs), number_op_second_int)
    }
}


impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
            (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

fn number_op_both_int(lhs: Value, rhs: Value,
                      float_op: fn(f64, f64) -> f64,
                      int_op: fn(i64, i64) -> Option<i64>) -> Value {
    if let Value::Int(lhs) = lhs {
        if let Value::Int(rhs) = rhs {
            return if let Some(sum) = int_op(lhs, rhs) {
                Value::Int(sum)
            } else {
                Value::Float(float_op(lhs as f64, rhs as f64))
            }
        }
    }
    Value::Float(float_op(lhs.as_num(), rhs.as_num()))
}

fn number_op_second_int(lhs: Value, rhs: Value,
                      float_op: fn(f64, f64) -> f64,
                      int_op: fn(i64, i64) -> Option<i64>) -> Value {
    let lhs = lhs.as_num();
    let lhs_is_int = lhs.fract() == 0.0;

    if let Value::Int(rhs) = rhs {
        if lhs_is_int {
            return if let Some(sum) = int_op(lhs as i64, rhs) {
                Value::Int(sum)
            } else {
                Value::Float(float_op(lhs, rhs as f64))
            }
        }
    }
    Value::Float(float_op(lhs, rhs.as_num()))
}

fn only_number_with_number_op(lhs: Value, rhs: Value,
                              float_op: fn(f64, f64) -> f64,
                              int_op: fn(i64, i64) -> Option<i64>,
    mode: fn(Value, Value, fn(f64, f64) -> f64, fn(i64, i64) -> Option<i64>) -> Value)
    -> Value {
    if let Value::String(_) = rhs {
        return Value::String(String::from("Nan"));
    }

    match lhs {
        Value::Float(_) => {
            mode(lhs, rhs, float_op, int_op)
        },
        Value::Int(_) => {
            mode(lhs, rhs, float_op, int_op)
        }
        Value::Bool(_) => {
            mode(lhs, rhs, float_op, int_op)
        }
        Value::String(_) => Value::String(String::from("Nan"))
    }
}

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Float(f) => write!(formatter, "Float({})", f),
            Value::Int(i) => write!(formatter, "Integer({})", i),
            Value::String(s) => write!(formatter, "String(\"{}\")", s),
            Value::Bool(b) => write!(formatter, "Bool({})", b),
        }
    }
}