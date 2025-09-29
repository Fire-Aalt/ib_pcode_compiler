use crate::data::diagnostic::{Diagnostic, ErrorType, LineInfo};
use crate::data::{NameHash, Value};
use std::collections::HashMap;
use crate::data::validator::Validator;

#[derive(Debug, Clone)]
pub struct StmtNode {
    pub line_info: LineInfo,
    pub stmt: Stmt,
}

#[derive(Debug, Clone)]
pub struct ExprNode {
    pub line_info: LineInfo,
    pub expr: Expr,
}

impl StmtNode {
    pub fn error(&self, error_type: ErrorType, message: &str) -> Result<Option<Value>, Diagnostic> {
        self.line_info.option_error(error_type, message)
    }
}

impl ExprNode {
    pub fn valid_error(&self, error_type: ErrorType, message: &str, validator: &mut Validator) -> Result<(), Diagnostic> {
        let err = self.line_info.diagnostic(error_type, message);
        validator.errors.push(err.clone());
        Err(err)
    }

    pub fn error(&self, error_type: ErrorType, message: &str) -> Result<Value, Diagnostic> {
        self.line_info.error(error_type, message)
    }

    pub fn diagnostic(&self, error_type: ErrorType, message: &str) -> Diagnostic {
        let err = self.line_info.diagnostic(error_type, message);
        err
    }

    pub fn compile_diagnostic(&self, error_type: ErrorType, message: &str, validator: &mut Validator) -> Diagnostic {
        let err = self.line_info.diagnostic(error_type, message);
        validator.errors.push(err.clone());
        err
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(NameHash),
    Data(Value),
    Array(Vec<ExprNode>),
    Unary(UnaryOp, Box<ExprNode>),
    BinOp(Box<ExprNode>, Operator, Box<ExprNode>),
    MethodCall(NameHash, Vec<ExprNode>),
    SubstringCall {
        expr: Box<ExprNode>,
        start: Box<ExprNode>,
        end: Box<ExprNode>,
    },
    LengthCall(Box<ExprNode>),
    ClassNew(NameHash, Vec<ExprNode>),
    Call {
        expr: Box<ExprNode>,
        fn_name: NameHash,
        params: Vec<ExprNode>,
    },
    Index(Box<ExprNode>, Box<ExprNode>),
    Input(Box<ExprNode>),
    Div(Box<ExprNode>, Box<ExprNode>),
}

#[derive(Debug, Clone)]
pub enum AssignTarget {
    Ident(NameHash),
    Array(ExprNode, ExprNode),
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

#[derive(Debug, Clone)]
pub struct Function {
    pub args: Vec<NameHash>,
    pub body: Vec<StmtNode>,
    pub returns: bool
}

#[derive(Debug, Clone)]
pub struct Class {
    pub functions: HashMap<NameHash, Function>,
    pub constructor: Constructor,
}

#[derive(Debug, Default, Clone)]
pub struct Constructor {
    pub constructors: Vec<(NameHash, ExprNode)>,
    pub args: Vec<NameHash>,
}
