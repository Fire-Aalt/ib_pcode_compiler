use crate::data::{NameHash, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct StmtNode {
    pub line: Line,
    pub stmt: Stmt,
}

#[derive(Debug)]
pub struct ExprNode {
    pub line: Line,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Diagnostic {
    pub line: Line,
    pub message: String,
}

impl StmtNode {
    pub fn error(&self, message: &str) -> Result<Option<Value>, Diagnostic> {
        Err(Diagnostic { line: self.line.clone(), message: message.to_string() })
    }
}

impl ExprNode {
    pub fn error(&self, message: &str) -> Result<Value, Diagnostic> {
        Err(Diagnostic { line: self.line.clone(), message: message.to_string() })
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub start_line: i32,
    pub end_line: i32,
    pub start_pos: u16,
    pub end_pos: u16,
}

#[derive(Debug)]
pub enum Stmt {
    Assign(AssignTarget, AssignOperator, ExprNode),
    Increment(AssignTarget),
    Decrement(AssignTarget),
    If {
        cond: ExprNode,
        then_branch: Vec<StmtNode>,
        elifs: Vec<(ExprNode, Vec<StmtNode>)>,
        else_branch: Option<Vec<StmtNode>>,
    },
    While(ExprNode, Vec<StmtNode>),
    For(NameHash, ExprNode, ExprNode, Vec<StmtNode>),
    Until(ExprNode, Vec<StmtNode>),
    Input(NameHash),
    Output(Vec<ExprNode>),
    Assert(ExprNode, ExprNode),
    FunctionDeclaration(NameHash),
    ClassDeclaration(NameHash),
    Expr(ExprNode),
    MethodReturn(ExprNode),
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
    Array(Vec<ExprNode>),
    Unary(UnaryOp, Box<ExprNode>),
    BinOp(Box<ExprNode>, Operator, Box<ExprNode>),
    MethodCall(NameHash, Vec<ExprNode>),
    SubstringCall { expr: Box<ExprNode>, start: Box<ExprNode>, end: Box<ExprNode> },
    ClassNew(NameHash, Vec<ExprNode>),
    Call { expr: Box<ExprNode>, fn_name: NameHash, params: Vec<ExprNode> },
    Index(Box<ExprNode>, Box<ExprNode>),
    Input(Box<ExprNode>),
    Div(Box<ExprNode>, Box<ExprNode>)
}

#[derive(Debug)]
pub enum AssignTarget {
    Ident(NameHash),
    Array(ExprNode, ExprNode)
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
    pub body: Vec<StmtNode>,
}

#[derive(Debug)]
pub struct Class {
    pub functions: HashMap<NameHash, Function>,
    pub constructor: Constructor,
}

#[derive(Debug, Default)]
pub struct Constructor {
    pub constructors: Vec<(NameHash, ExprNode)>,
    pub args: Vec<NameHash>,
}