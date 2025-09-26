use crate::ast_nodes::Value;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Env {
    pub arrays: AllocatedLookupMap<VecDeque<Value>>,
    pub locals: AllocatedLookupMap<LocalEnv>,
    pub local_ids_stack: Vec<usize>,
    pub mode: EnvMode,
}

#[derive(Debug)]
pub struct AllocatedLookupMap<T> {
    pub map: HashMap<usize, T>,
    next_id: usize,
}

impl<T> AllocatedLookupMap<T> {
    pub fn new() -> AllocatedLookupMap<T> {
        Self {
            map: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn alloc(&mut self, item: T) -> usize {
        let id = self.next_id;
        self.map.insert(id, item);
        self.next_id += 1;
        id
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.map.get_mut(&id)
    }
}

#[derive(Debug)]
pub struct LocalEnv {
    pub class_name: String,
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
        let mut e = Self { arrays: AllocatedLookupMap::new(), locals: AllocatedLookupMap::new(), local_ids_stack: Vec::new(), mode };
        e.create_local_env(""); // global env
        e.push_local_env(0);
        e
    }

    pub fn record_log(logs: &mut VecDeque<String>, log: String) {
        logs.push_back(log);
    }

    pub fn create_local_env(&mut self, class_name: &str) -> usize {
        self.locals.alloc(LocalEnv::new(class_name))
    }

    pub fn create_array(&mut self, array: VecDeque<Value>) -> usize {
        self.arrays.alloc(array)
    }

    pub fn get_array(&self, id: usize) -> &VecDeque<Value> {
        self.arrays.get(id).unwrap()
    }

    pub fn get_array_mut(&mut self, id: usize) -> &mut VecDeque<Value> {
        self.arrays.get_mut(id).unwrap()
    }

    pub fn push_local_env(&mut self, id: usize) {
        self.local_ids_stack.push(id);
    }

    pub fn pop_local_env(&mut self) {
        self.local_ids_stack.pop();
    }

    pub fn push_scope(&mut self) {
        self.get_local_env_mut().push_scope();
    }

    pub fn pop_scope(&mut self) {
        self.get_local_env_mut().pop_scope();
    }

    pub fn assign(&mut self, name: &str, val: Value) {
        self.get_local_env_mut().assign(name, val);
    }

    pub fn define(&mut self, name: String, val: Value) {
        self.get_local_env_mut().define(name, val);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.get_local_env().get(name)
    }

    pub fn get_local_env(&self) -> &LocalEnv {
        self.locals.get(*self.local_ids_stack.last().unwrap()).unwrap()
    }

    fn get_local_env_mut(&mut self) -> &mut LocalEnv {
        self.locals.get_mut(*self.local_ids_stack.last().unwrap()).unwrap()
    }
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

    /// Lookup (clones the value)
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
