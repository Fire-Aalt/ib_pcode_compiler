use crate::compiler::Rule;
use crate::data::ast_nodes::{Class, Constructor, Function, StmtNode};
use crate::data::diagnostic::LineInfo;
use crate::data::name_hash::{with_name_map, NameHash};
use crate::data::Value;
use crate::env::Env;
use pest::iterators::Pair;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{DefaultHasher, Hash, Hasher};

pub mod builder;
pub mod evaluator;
mod validator;

pub struct AST {
    pub source: String,
    pub user_code_start_line: u32,
    pub nodes: Vec<StmtNode>,
    pub hash_to_name_map: HashMap<NameHash, String>,
    pub statics: HashSet<NameHash>,
    class_map: HashMap<NameHash, Class>,
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        with_name_map(&self.hash_to_name_map, || {
            write!(f, "{:#?}\n{:#?}", self.class_map, self.nodes)
        })
    }
}

impl AST {
    pub fn new(source: String, user_code_start_line: u32) -> Self {
        let mut ast = Self {
            source,
            user_code_start_line,
            nodes: Vec::new(),
            class_map: HashMap::new(),
            hash_to_name_map: HashMap::new(),
            statics: HashSet::new(),
        };
        let main = ast.main_hash();
        ast.add_class(
            main,
            Class {
                line_info: LineInfo::default(),
                functions: HashMap::new(),
                constructor: Constructor::default(),
                is_static: false,
            },
        );
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
        self.class_map
            .get_mut(hash)
            .unwrap()
            .functions
            .insert(name_hash, function);
    }

    pub fn get_name(&self, name_hash: &NameHash) -> &str {
        self.hash_to_name_map.get(name_hash).unwrap().as_str()
    }

    pub fn get_class(&self, name_hash: &NameHash) -> Option<&Class> {
        self.class_map.get(name_hash)
    }

    pub fn get_function(
        &self,
        class_name_hash: &NameHash,
        fn_name_hash: &NameHash,
    ) -> Option<&Function> {
        self.class_map
            .get(class_name_hash)
            .unwrap()
            .functions
            .get(fn_name_hash)
    }

    pub fn hash(&mut self, string: &str) -> NameHash {
        let name_hash = hash(string);
        self.hash_to_name_map.insert(name_hash.clone(), string.to_string());
        name_hash
    }

    pub fn hash_static(&mut self, string: &str) -> NameHash {
        let name_hash = hash(string);
        self.hash_to_name_map.insert(name_hash.clone(), string.to_string());
        self.statics.insert(name_hash.clone());
        name_hash
    }

    pub fn format_val(&self, val: &Value, output: &mut String, env: &Env) {
        match val {
            Value::Number(n) => {
                if n.abs() > 100000000000000000000.0 {
                    output.push_str(&format!("{:e}", n));
                } else {
                    output.push_str(&format!("{}", n));
                }
            }
            Value::String(s) => output.push_str(s.trim()),
            Value::Bool(b) => output.push_str(&b.to_string()),
            Value::ArrayId(id) => {
                for (i, array_val) in env.get_array(id).iter().enumerate() {
                    if i > 0 {
                        output.push(',');
                    }

                    match array_val {
                        Value::Undefined => {}
                        _ => self.format_val(array_val, output, env),
                    }
                }
            }
            Value::InstanceId(id) => {
                let local = env.get_local_env_at(id);

                output.push_str(self.get_name(&local.class_name));
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
            }
            Value::Undefined => output.push_str("Undefined"),
        }
    }

    pub fn as_line_info(&self, pair: &Pair<Rule>) -> LineInfo {
        let span = pair.as_span();
        let (start_line, start_col) = pair.line_col();
        let (_end_line, mut end_col) = span.end_pos().line_col();


        let first_line_end_col = match span.lines().next() {
            Some(line) => line.chars().count() - 1,
            None => end_col,
        };

        if first_line_end_col > end_col {
            end_col = first_line_end_col;
        }

        LineInfo {
            start_line: start_line as u32,
            start_col: start_col as u16,
            end_col: end_col as u16,
        }
    }
}

pub fn hash(string: &str) -> NameHash {
    let mut hasher = DefaultHasher::new();

    let name_hash = match string.strip_prefix("this.") {
        Some(stripped) => {
            stripped.hash(&mut hasher);
            NameHash {
                hash: hasher.finish(),
                this_keyword: true
            }
        }
        None => {
            string.hash(&mut hasher);
            NameHash {
                hash: hasher.finish(),
                this_keyword: false
            }
        }
    };
    name_hash
}

pub fn main_hash() -> NameHash {
    hash("main")
}
