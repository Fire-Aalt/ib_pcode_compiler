use std::collections::HashMap;
use crate::data::{NameHash, Value};

#[derive(Debug)]
pub enum Stmt {
    Assign(AssignTarget, AssignOperator, Expr),
    Increment(AssignTarget),
    Decrement(AssignTarget),
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        elifs: Vec<(Expr, Vec<Stmt>)>,
        else_branch: Option<Vec<Stmt>>,
    },
    While(Expr, Vec<Stmt>),
    For(NameHash, Expr, Expr, Vec<Stmt>),
    Until(Expr, Vec<Stmt>),
    Input(NameHash),
    Output(Vec<Expr>),
    Assert(Expr, Expr),
    FunctionDeclaration(NameHash),
    ClassDeclaration(NameHash),
    Expr(Expr),
    MethodReturn(Expr),
    EOI,
}

#[derive(Debug)]
pub enum Expr {
    Ident(NameHash),
    Data(Value),
    Array(Vec<Expr>),
    Unary(UnaryOp, Box<Expr>),
    BinOp(Box<Expr>, Operator, Box<Expr>),
    MethodCall(NameHash, Vec<Expr>),
    SubstringCall { expr: Box<Expr>, start: Box<Expr>, end: Box<Expr> },
    ClassNew(NameHash, Vec<Expr>),
    Call { expr: Box<Expr>, fn_name: NameHash, params: Vec<Expr> },
    Index(Box<Expr>, Box<Expr>),
    Input(Box<Expr>),
    Div(Box<Expr>, Box<Expr>)
}

#[derive(Debug)]
pub enum AssignTarget {
    Ident(NameHash),
    Array(Expr, Expr)
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    IntDivide,
    Modulo,
    Power,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum AssignOperator {
    Assign,
    AssignAdd,
    AssignSubtract,
    AssignMultiply,
    AssignDivide,
}

#[derive(Debug)]
pub struct Function {
    pub args: Vec<NameHash>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Class {
    pub functions: HashMap<NameHash, Function>,
    pub constructor: Constructor,
}

#[derive(Debug, Default)]
pub struct Constructor {
    pub constructors: Vec<(NameHash, Expr)>,
    pub args: Vec<NameHash>,
}