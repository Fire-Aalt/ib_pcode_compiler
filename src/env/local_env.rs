use std::collections::HashMap;
use crate::data::{NameHash, Value};

#[derive(Debug)]
pub struct LocalEnv {
    pub class_name: NameHash,
    pub scopes: Vec<HashMap<NameHash, Value>>,
}

impl LocalEnv {
    pub fn new(class_name_hash: NameHash) -> Self {
        let mut e = Self { class_name: class_name_hash, scopes: Vec::new() };
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
    pub fn define(&mut self, name_hash: &NameHash, val: Value) {
        if let Some(top) = self.scopes.last_mut() {
            top.insert(name_hash.clone(), val);
        } else {
            panic!("no scope to define variable");
        }
    }

    /// Undefine in current (top) scope
    pub fn undefine(&mut self, name_hash: &NameHash) {
        if let Some(top) = self.scopes.last_mut() {
            top.remove(name_hash);
        } else {
            panic!("no scope to define variable");
        }
    }

    /// Assign to nearest existing scope containing the var, or create in current scope
    pub fn assign(&mut self, name_hash: &NameHash, val: Value) {
        if name_hash.this_keyword {
            self.scopes.first_mut().unwrap().insert(name_hash.clone(), val);
            return;
        }

        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name_hash.clone()) {
                scope.insert(name_hash.clone(), val);
                return;
            }
        }
        // not found -> create in current scope
        if let Some(top) = self.scopes.last_mut() {
            top.insert(name_hash.clone(), val);
        } else {
            panic!("no scope to assign variable");
        }
    }

    pub fn get(&self, name_hash: &NameHash) -> Option<Value> {
        if name_hash.this_keyword {
            if let Some(v) = self.scopes.first().unwrap().get(name_hash) {
                return Some(v.clone());
            }
            return None
        }

        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name_hash) {
                return Some(v.clone());
            }
        }
        None
    }
}
