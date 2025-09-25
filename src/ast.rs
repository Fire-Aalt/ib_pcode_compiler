use crate::ast_nodes::{Class, Function, Stmt};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Default)]
pub struct AST {
    pub statements: Vec<Stmt>,
    pub function_map: HashMap<String, Function>,
    pub class_map: HashMap<String, Class>,
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.statements)
    }
}
