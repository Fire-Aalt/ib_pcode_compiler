use crate::ast::AST;
use crate::data::diagnostic::{Diagnostic, LineInfo};
use crate::data::{NameHash, Value};
use crate::env::Env;
use std::collections::HashMap;

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

impl ExprNode {
    pub fn eval_as_num(&self, ast: &AST, env: &mut Env) -> Result<f64, Diagnostic> {
        ast.eval_expr(self, env)?.as_num(&self.line_info)
    }

    pub fn eval_as_bool_unsafe(&self, ast: &AST, env: &mut Env) -> Result<bool, Diagnostic> {
        Ok(ast.eval_expr(self, env)?.as_bool_unsafe())
    }

    pub fn eval_as_str(&self, ast: &AST, env: &mut Env) -> Result<String, Diagnostic> {
        Ok(ast.eval_expr(self, env)?.as_string())
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
    BinOp(Box<ExprNode>, Operand, Box<ExprNode>),
    LocalMethodCall(NameHash, Vec<ExprNode>),
    StaticMethodCall(NameHash, NameHash, Vec<ExprNode>),
    SubstringCall {
        expr: Box<ExprNode>,
        start: Box<ExprNode>,
        end: Box<ExprNode>,
    },
    LengthCall(Box<ExprNode>),
    ClassNew(NameHash, Vec<ExprNode>),
    ClassMethodCall {
        expr: Box<ExprNode>,
        fn_name: NameHash,
        params: Vec<ExprNode>,
    },
    Index(Box<ExprNode>, Box<ExprNode>),
    Input(Box<ExprNode>),
    Div(Box<ExprNode>, Box<ExprNode>),
    MathRandom,
}

#[derive(Debug, Clone)]
pub enum AssignTarget {
    Ident(NameHash),
    Array(ExprNode, ExprNode),
}

#[derive(Debug, Clone)]
pub enum Operand {
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

impl Operand {
    pub fn error_fmt(&self) -> String {
        let str = match self {
            Operand::Add => "add",
            Operand::Subtract => "subtract",
            Operand::Multiply => "multiply",
            Operand::Divide => "divide",
            Operand::IntDivide => "integer divide",
            Operand::Modulo => "modulo",
            Operand::Power => "power",
            Operand::Greater => "greater",
            Operand::Less => "less",
            Operand::GreaterEqual => "greater equal",
            Operand::LessEqual => "less equal",
            Operand::Equal => "equal",
            Operand::NotEqual => "not equal",
            Operand::And => "and",
            Operand::Or => "or",
        };
        String::from(str)
    }
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
    pub returns: bool,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub line_info: LineInfo,
    pub functions: HashMap<NameHash, Function>,
    pub constructor: Constructor,
    pub is_static: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Constructor {
    pub line_info: LineInfo,
    pub constructors: Vec<(NameHash, ExprNode)>,
    pub args: Vec<NameHash>,
}
