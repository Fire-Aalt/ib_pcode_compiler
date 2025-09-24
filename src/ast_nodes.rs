pub(crate) use crate::ast_nodes::value::Value;

pub mod value;

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(String, AssignOperator, Expr),
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
    MethodDeclaration(String, Vec<String>),
    MethodCall(String, Vec<Box<Expr>>),
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
    MethodCall(String, Vec<Box<Expr>>),
    Call(Box<Expr>, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Input(Box<Expr>),
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

#[derive(Clone, Debug)]
pub struct MethodDef {
    pub args: Vec<String>,
    pub body: Vec<Stmt>,
}
