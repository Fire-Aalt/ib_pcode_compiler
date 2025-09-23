use std::collections::{HashMap, VecDeque};
use crate::ast_nodes::Value;

#[derive(Debug)]
pub struct Env {
    scopes: Vec<HashMap<String, Value>>,
    pub test_mode: bool,
    pub mock_inputs: VecDeque<String>,
    pub logs: VecDeque<String>,
}

impl Env {
    pub fn new(mock_inputs: VecDeque<String>, test_mode: bool) -> Self {
        let mut e = Self { scopes: Vec::new(), test_mode, logs: VecDeque::new(), mock_inputs };
        e.push_scope(); // global scope
        e
    }

    pub fn record_log(&mut self, log: String) {
        self.logs.push_back(log);
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

    /// Lookup (clones the value)
    pub fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
}
    
