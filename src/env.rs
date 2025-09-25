use crate::ast_nodes::Value;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Env {
    pub locals: HashMap<usize, LocalEnv>,
    pub local_ids_stack: Vec<usize>,
    next_local_id: usize,
    pub mode: EnvMode,
}

#[derive(Debug)]
pub struct LocalEnv {
    pub scopes: Vec<HashMap<String, Value>>,
}

#[derive(Debug)]
pub enum EnvMode {
    Release,
    Test {
        mock_inputs: VecDeque<String>,
        logs: VecDeque<String>,
    },
}

impl Env {
    pub fn release() -> Self {
        let mode = EnvMode::Release;
        Env::new(mode)
    }

    pub fn test(mock_inputs: VecDeque<String>) -> Self {
        let mode = EnvMode::Test {
            mock_inputs,
            logs: VecDeque::new(),
        };
        Env::new(mode)
    }

    pub fn new(mode: EnvMode) -> Self {
        let mut e = Self { locals: HashMap::new(), mode, local_ids_stack: Vec::new(), next_local_id: 0 };
        e.create_local(); // global env
        e.push_local(0);
        e
    }

    pub fn record_log(logs: &mut VecDeque<String>, log: String) {
        logs.push_back(log);
    }

    pub fn create_local(&mut self) -> usize {
        let id = self.next_local_id;
        self.locals.insert(id, LocalEnv::new());
        self.next_local_id += 1;
        id
    }

    pub fn push_local(&mut self, id: usize) {
        self.local_ids_stack.push(id);
    }

    pub fn pop_local(&mut self) {
        self.local_ids_stack.pop();
    }

    pub fn push_scope(&mut self) {
        self.get_local_mut().push_scope();
    }

    pub fn pop_scope(&mut self) {
        self.get_local_mut().pop_scope();
    }

    pub fn assign(&mut self, name: &str, val: Value) {
        self.get_local_mut().assign(name, val);
    }

    pub fn define(&mut self, name: String, val: Value) {
        self.get_local_mut().define(name, val);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.get_local().get(name)
    }

    pub fn get_local(&self) -> &LocalEnv {
        self.locals.get(self.local_ids_stack.last().unwrap()).unwrap()
    }

    fn get_local_mut(&mut self) -> &mut LocalEnv {
        self.locals.get_mut(self.local_ids_stack.last().unwrap()).unwrap()
    }
}


impl LocalEnv {
    pub fn new() -> Self {
        let mut e = Self { scopes: Vec::new() };
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
