use crate::ast::AST;
use crate::data::diagnostic::{Diagnostic, LineInfo};
use crate::data::{NameHash, Value};
use crate::env::Env;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct StmtNode {
    pub line_info: LineInfo,
    pub stmt: Stmt,
}

#[derive(Debug)]
pub struct ExprNode {
    pub line_info: LineInfo,
    pub expr: Expr,
}

impl ExprNode {
    /// # Safety
    /// Only supports Number, Bool and Strings
    pub unsafe fn eval_as_bool_unchecked(
        &self,
        ast: &AST,
        env: &mut Env,
    ) -> Result<bool, Diagnostic> {
        unsafe { Ok(ast.eval_expr(self, env)?.as_bool_unchecked()) }
    }
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

#[derive(Debug)]
pub enum Expr {
    Ident(NameHash),
    Data(Value),
    Array(Vec<ExprNode>),
    Unary(UnaryOp, Box<ExprNode>),
    BinOp(Box<ExprNode>, Operand, Box<ExprNode>),
    LocalMethodCall(NameHash, Vec<ExprNode>),
    StaticMethodCall(LineInfo, NameHash, NameHash, Vec<ExprNode>),
    StaticGetVar(LineInfo, NameHash, NameHash),
    ClassGetVar(Box<ExprNode>, LineInfo, NameHash),
    ClassNew(NameHash, Vec<ExprNode>),
    ClassMethodCall {
        expr: Box<ExprNode>,
        fn_line: LineInfo,
        fn_name: NameHash,
        params: Vec<ExprNode>,
    },
    Index(Box<ExprNode>, Box<ExprNode>),
    NativeMethodCall(NativeMethod, Option<Box<ExprNode>>, LineInfo, Vec<ExprNode>),
}

#[derive(Debug)]
pub enum NativeMethod {
    Div,
    Input,
    MathRandom,
    SubstringCall,
    LengthCall,
}

#[derive(Debug)]
pub enum AssignTarget {
    Ident(NameHash),
    Array(ExprNode, ExprNode),
}

#[derive(Debug)]
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
    pub returns: bool,
}

#[derive(Debug)]
pub struct Class {
    pub line_info: LineInfo,
    pub functions: HashMap<NameHash, Function>,
    pub public_vars: HashSet<NameHash>,
    pub constructor: Constructor,
    pub is_static: bool,
}

#[derive(Debug, Default)]
pub struct Constructor {
    pub line_info: LineInfo,
    pub constructors: Vec<(NameHash, ExprNode)>,
    pub args: Vec<NameHash>,
}
