use crate::ast_nodes::Value;
use crate::env::allocated_lookup_map::AllocatedLookupMap;
use crate::env::local_env::LocalEnv;
use std::collections::VecDeque;

mod allocated_lookup_map;
mod local_env;

#[derive(Debug)]
pub struct Env {
    pub arrays: AllocatedLookupMap<VecDeque<Value>>,
    pub locals: AllocatedLookupMap<LocalEnv>,
    pub local_ids_stack: Vec<usize>,
    pub mode: EnvMode,
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

    pub fn get_array(&self, id: &usize) -> &VecDeque<Value> {
        self.arrays.get(id).unwrap()
    }

    pub fn get_array_mut(&mut self, id: &usize) -> &mut VecDeque<Value> {
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
        self.locals.get(self.local_ids_stack.last().unwrap()).unwrap()
    }

    fn get_local_env_mut(&mut self) -> &mut LocalEnv {
        self.locals.get_mut(self.local_ids_stack.last().unwrap()).unwrap()
    }
}