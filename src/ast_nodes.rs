use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(String, AssignOperator, Expr),
    If(Expr, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    For(String, Expr, Expr, Vec<Stmt>),
    Output(Vec<Expr>),
    MethodDeclaration(String, Vec<String>),
    MethodCall(String, Vec<Box<Expr>>),
    MethodReturn(Expr),
    EOI
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    Data(Value),
    BinOp(Box<Expr>, Operator, Box<Expr>), // Has to be boxed to avoid recursion in the enum definition
    MethodCall(String, Vec<Box<Expr>>),
    Input(Box<Expr>),
}

impl From<Box<Expr>> for Expr {
    fn from(value: Box<Expr>) -> Self {
        value.into()
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone)]
pub enum AssignOperator {
    Assign,
    AssignAdd,
    AssignSubtract,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct MethodDef {
    pub args: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(lhs) => {
                match rhs {
                    Value::Number(rhs) => Value::Number(lhs + rhs),
                    Value::String(rhs) => Value::String(format!("{}{}", lhs, rhs))
                }
            },
            Value::String(lhs) => Value::String(format!("{}{}", lhs, rhs))
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(lhs) => {
                match rhs {
                    Value::Number(rhs) => Value::Number(lhs - rhs),
                    Value::String(_) => Value::String(String::from("Nan"))
                }
            },
            Value::String(_) => Value::String(String::from("Nan"))
        }
    }
}

impl Value {
    pub fn as_num(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::String(s) => 0.0,
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            Value::String(n) => n.clone(),
            _ => unreachable!()
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::String(s) => write!(f, "String(\"{}\")", s),
        }
    }
}