use crate::ast_nodes::{Function, Stmt};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Default)]
pub struct AST {
    pub statements: Vec<Stmt>,
    pub method_map: HashMap<String, Function>,
    pub class_map: HashMap<String, Function>,
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.statements)
    }
}
