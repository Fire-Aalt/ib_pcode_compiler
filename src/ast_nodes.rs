use std::collections::HashMap;
pub(crate) use crate::ast_nodes::value::Value;

pub mod value;

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(String, Option<Expr>, AssignOperator, Expr),
    Increment(String),
    Decrement(String),
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        elifs: Vec<(Expr, Vec<Stmt>)>,
        else_branch: Option<Vec<Stmt>>,
    },
    While(Expr, Vec<Stmt>),
    For(String, Expr, Expr, Vec<Stmt>),
    Until(Expr, Vec<Stmt>),
    Input(String),
    Output(Vec<Expr>),
    Assert(Expr, Expr),
    FunctionDeclaration(String),
    ClassDeclaration(String),
    MethodCall(String, Vec<Expr>),
    MethodReturn(Expr),
    EOI,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    Data(Value),
    Array(Vec<Expr>),
    Unary(UnaryOp, Box<Expr>),
    BinOp(Box<Expr>, Operator, Box<Expr>),
    MethodCall(String, Vec<Expr>),
    SubstringCall { expr: Box<Expr>, start: Box<Expr>, end: Box<Expr> },
    ClassNew(String, Vec<Expr>),
    Call { expr: Box<Expr>, fn_name: String, params: Vec<Expr> },
    Index(Box<Expr>, Box<Expr>),
    Input(Box<Expr>),
    Div(Box<Expr>, Box<Expr>)
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum AssignOperator {
    Assign,
    AssignAdd,
    AssignSubtract,
    AssignMultiply,
    AssignDivide,
}

#[derive(Debug)]
pub struct Function {
    pub args: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Class {
    pub functions: HashMap<String, Function>,
    pub constructor: Constructor,
}

#[derive(Debug)]
pub struct Constructor {
    pub args: Vec<String>,
    pub vars: Vec<(String, Expr)>,
}