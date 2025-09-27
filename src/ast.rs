use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{DefaultHasher, Hash, Hasher};
use crate::data::ast_nodes::{Class, Constructor, Function, Stmt};
use crate::data::name_hash::NameHash;
use crate::data::Value;
use crate::env::Env;

pub mod builder;
pub mod evaluator;

pub struct AST {
    pub statements: Vec<Stmt>,
    class_map: HashMap<NameHash, Class>,
    hash_to_name_map: HashMap<NameHash, String>
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}\n{:#?}", self.class_map, self.statements)
    }
}

impl AST {
    pub fn new() -> Self {
        let mut ast = Self { statements: Vec::new(), class_map: HashMap::new(), hash_to_name_map: HashMap::new() };
        let main = ast.main_hash();
        ast.add_class(main, Class { functions: HashMap::new(), constructor: Constructor::default() });
        ast
    }

    pub fn main_hash(&mut self) -> NameHash {
        self.hash("main")
    }

    pub fn add_class(&mut self, name_hash: NameHash, class: Class) {
        self.class_map.insert(name_hash, class);
    }

    pub fn add_function(&mut self, name_hash: NameHash, function: Function) {
        let hash = &self.main_hash();
        self.class_map.get_mut(hash).unwrap().functions.insert(name_hash, function);
    }

    pub fn get_name(&self, name_hash: &NameHash) -> &str {
        self.hash_to_name_map.get(name_hash).unwrap().as_str()
    }

    pub fn get_class(&self, name_hash: &NameHash) -> &Class {
        self.class_map.get(name_hash).unwrap()
    }

    pub fn get_function(&self, class_name_hash: &NameHash, fn_name_hash: &NameHash) -> &Function {
        self.class_map.get(class_name_hash).unwrap().functions.get(fn_name_hash).unwrap()
    }

    pub fn hash(&mut self, string: &str) -> NameHash {
        let name_hash = hash(string);
        self.hash_to_name_map.insert(name_hash.clone(), string.to_string());
        name_hash
    }

    pub fn format_val(&self, val: &Value, output: &mut String, env: &Env) {
        match val {
            Value::Number(n) => {
                if n.abs() > 100000000000000000000.0 {
                    output.push_str(&format!("{:e}", n));
                }
                else {
                    output.push_str(&format!("{}", n));
                }
            },
            Value::String(s) => output.push_str(s.trim()),
            Value::Bool(b) => output.push_str(&b.to_string()),
            Value::Array(id) => {
                for (i, array_val) in env.get_array(id).iter().enumerate() {
                    if i > 0 {
                        output.push(',');
                    }

                    self.format_val(array_val, output, env);
                }
            },
            Value::Instance(id) => {
                let local = env.get_local_env_at(id);

                output.push_str(self.get_name(&local.class_name_hash));
                output.push_str(": [");

                for (i, (name, val)) in local.scopes.first().unwrap().iter().enumerate() {
                    if i > 0 {
                        output.push(',');
                    }

                    output.push_str(self.get_name(name));
                    output.push_str(": ");
                    self.format_val(val, output, env);
                }
                output.push(']');
            },
        }
    }
}

pub fn hash(string: &str) -> NameHash {
    let mut hasher = DefaultHasher::new();

    let name_hash = match string.strip_prefix("this.") {
        Some(stripped) => {
            stripped.hash(&mut hasher);
            NameHash { hash: hasher.finish(), this_keyword: true }
        },
        None => {
            string.hash(&mut hasher);
            NameHash { hash: hasher.finish(), this_keyword: false }
        }
    };
    name_hash
}

pub fn main_hash() -> NameHash {
    hash("main")
}

