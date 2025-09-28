use crate::data::{NameHash, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct AstNode {
    pub line: Line,
    pub stmt: Stmt,
}

#[derive(Debug)]
pub struct Line {
    pub string: String,
    pub line: i32
}

#[derive(Debug)]
pub enum Stmt {
    Assign(AssignTarget, AssignOperator, Expr),
    Increment(AssignTarget),
    Decrement(AssignTarget),
    If {
        cond: Expr,
        then_branch: Vec<AstNode>,
        elifs: Vec<(Expr, Vec<AstNode>)>,
        else_branch: Option<Vec<AstNode>>,
    },
    While(Expr, Vec<AstNode>),
    For(NameHash, Expr, Expr, Vec<AstNode>),
    Until(Expr, Vec<AstNode>),
    Input(NameHash),
    Output(Vec<Expr>),
    Assert(Expr, Expr),
    FunctionDeclaration(NameHash),
    ClassDeclaration(NameHash),
    Expr(Expr),
    MethodReturn(Expr),
    EOI,
}

/*impl Stmt {
    pub fn to_node(self, pair: &Pair<Rule>) -> AstNode {
        AstNode { line: pair.as_span().as_str().to_string(), stmt: self }
    }
}*/

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
    pub body: Vec<AstNode>,
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