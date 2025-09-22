use crate::ast_nodes::{MethodDef, Stmt};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct AST {
    pub statements: Vec<Stmt>,
    pub method_map: HashMap<String, MethodDef>,
}

impl Default for AST {
    fn default() -> Self {
        AST { statements: Vec::new(), method_map: HashMap::new() }
    }
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.statements)
    }
}