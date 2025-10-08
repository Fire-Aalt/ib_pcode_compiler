use crate::common::{to_bool_num, to_bool_str};
use crate::compiler::errors::{diagnostic, unsupported_operand_error};
use crate::data::ast_nodes::Operand;
use crate::data::diagnostic::{Diagnostic, ErrorType, LineInfo};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    ArrayId(usize),
    InstanceId(usize),
    Undefined,
}

impl Value {
    pub fn error_fmt(&self) -> String {
        println!("{}", self);
        match self {
            Value::Number(n) => format!("Number({})", n),
            Value::Bool(b) => format!("Boolean({})", b),
            Value::String(s) => format!("String({})", s),
            Value::ArrayId(_) => "Array(...)".to_string(),
            Value::InstanceId(_) => "ClassInstance(...)".to_string(),
            Value::Undefined => "Undefined".to_string(),
        }
    }

    pub fn fmt(&self) -> String {
        match self {
            Value::Number(n) => format!("{}", n),
            Value::Bool(b) => format!("{}", b),
            Value::String(s) => s.to_string(),
            Value::ArrayId(_) => "Array(...)".to_string(),
            Value::InstanceId(_) => "ClassInstance(...)".to_string(),
            Value::Undefined => "Undefined".to_string(),
        }
    }

    pub fn neg(self, line_info: &LineInfo) -> Result<Self, Diagnostic> {
        match self {
            Value::Number(f) => Ok(Value::Number(-f)),
            Value::Bool(b) => Ok(Value::Number(if b { -1.0 } else { 0.0 })),
            _ => Err(diagnostic(
                line_info,
                ErrorType::InvalidType,
                format!("cannot apply `negate` operand on `{}`", self.error_fmt()),
                "",
            )),
        }
    }

    pub fn not(self, line_info: &LineInfo) -> Result<Self, Diagnostic> {
        match self {
            Value::Number(f) => Ok(Value::Bool(f == 0.0)),
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => Err(diagnostic(
                line_info,
                ErrorType::InvalidType,
                format!("cannot apply `not` operand on `{}`", self.error_fmt()),
                "",
            )),
        }
    }

    pub fn add(self, line_info: &LineInfo, rhs: Self) -> Result<Self, Diagnostic> {
        match self {
            Value::String(lhs) => Ok(Value::String(format!("{}{}", lhs, rhs.fmt()))),
            _ => match rhs {
                Value::String(rhs) => Ok(Value::String(format!("{}{}", self.fmt(), rhs))),
                _ => Ok(Value::Number(
                    self.as_num(line_info)? + rhs.as_num(line_info)?,
                )),
            },
        }
    }

    pub fn sub(self, line_info: &LineInfo, rhs: Self) -> Result<Value, Diagnostic> {
        self.only_number_with_number_op(line_info, Operand::Subtract, rhs, |lhs, rhs| lhs - rhs)
    }

    pub fn mul(self, line_info: &LineInfo, rhs: Self) -> Result<Value, Diagnostic> {
        self.only_number_with_number_op(line_info, Operand::Multiply, rhs, |lhs, rhs| lhs * rhs)
    }

    pub fn div(self, line_info: &LineInfo, rhs: Self) -> Result<Value, Diagnostic> {
        self.only_number_with_number_op(line_info, Operand::Divide, rhs, |lhs, rhs| lhs / rhs)
    }

    pub fn only_number_with_number_op(
        self,
        line_info: &LineInfo,
        operand: Operand,
        rhs: Self,
        op: fn(f64, f64) -> f64,
    ) -> Result<Value, Diagnostic> {
        if !matches!(self, Value::Number(_) | Value::Bool(_)) {
            return Err(unsupported_operand_error(line_info, &self, &operand, &rhs));
        }
        if !matches!(rhs, Value::Number(_) | Value::Bool(_)) {
            return Err(unsupported_operand_error(line_info, &self, &operand, &rhs));
        }

        Ok(Value::Number(op(
            self.as_num(line_info)?,
            rhs.as_num(line_info)?,
        )))
    }

    pub fn as_num(&self, line_info: &LineInfo) -> Result<f64, Diagnostic> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::Bool(b) => {
                if *b {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            _ => Err(diagnostic(
                line_info,
                ErrorType::InvalidType,
                format!("cannot convert `{}` to a number", self.error_fmt()),
                "",
            )),
        }
    }

    /// # Safety
    /// Can only convert Number and Bool into a number, otherwise panics
    pub unsafe fn as_num_unchecked(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            _ => panic!("{}", self.error_fmt()),
        }
    }

    /// # Safety
    /// Can only convert Number, Bool and String into a bool, otherwise panics
    pub unsafe fn as_bool_unchecked(&self) -> bool {
        match self {
            Value::Number(n) => to_bool_num(*n),
            Value::Bool(b) => *b,
            Value::String(s) => to_bool_str(s),
            _ => panic!("{}", self.error_fmt()),
        }
    }

    pub fn as_bool(&self, line_info: &LineInfo) -> Result<bool, Diagnostic> {
        match self {
            Value::Number(n) => Ok(*n != 0.0),
            Value::Bool(b) => Ok(*b),
            _ => Err(diagnostic(
                line_info,
                ErrorType::InvalidType,
                format!("cannot convert `{}` to a boolean", self.error_fmt()),
                "",
            )),
        }
    }

    pub fn as_string(&self) -> String {
        self.fmt()
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

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(f) => write!(formatter, "Number({})", f),
            Value::String(s) => write!(formatter, "String(\"{}\")", s),
            Value::Bool(b) => write!(formatter, "Bool({})", b),
            Value::ArrayId(id) => write!(formatter, "Array(Id: {})", id),
            Value::InstanceId(id) => write!(formatter, "Instance(Id: {})", id),
            Value::Undefined => write!(formatter, "Undefined"),
        }
    }
}
