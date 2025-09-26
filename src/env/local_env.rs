use std::collections::HashMap;
use crate::ast_nodes::Value;

#[derive(Debug)]
pub struct LocalEnv {
    pub class_name: String,
    pub scopes: Vec<HashMap<String, Value>>,
}

impl LocalEnv {
    pub fn new(class_name: &str) -> Self {
        let mut e = Self { class_name: class_name.to_string(), scopes: Vec::new() };
        e.push_scope(); // top scope
        e
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop().expect("popping empty scope stack");
    }

    /// Define in current (top) scope
    pub fn define(&mut self, name: String, val: Value) {
        if let Some(top) = self.scopes.last_mut() {
            top.insert(name, val);
        } else {
            panic!("no scope to define variable");
        }
    }

    /// Assign to nearest existing scope containing the var, or create in current scope
    pub fn assign(&mut self, name: &str, val: Value) {
        if name.starts_with("this.") {
            self.scopes.first_mut().unwrap().insert(name.to_string(), val);
            return;
        }

        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), val);
                return;
            }
        }
        // not found -> create in current scope
        if let Some(top) = self.scopes.last_mut() {
            top.insert(name.to_string(), val);
        } else {
            panic!("no scope to assign variable");
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if name.starts_with("this.") {
            if let Some(v) = self.scopes.first().unwrap().get(name) {
                return Some(v.clone());
            }
            return None
        }

        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
}
