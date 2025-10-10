use crate::compiler::Rule;
use crate::data::ast_nodes::{Class, Constructor, Function, StmtNode};
use crate::data::diagnostic::{ErrorType, LineInfo};
use crate::data::name_hash::{NameHash, with_name_map};
use crate::data::{Validator, Value};
use crate::env::Env;
use pest::iterators::Pair;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};

pub mod builder;
pub mod evaluator;
mod hasher;
mod validator;
use crate::compiler::errors::{compile_error, diagnostic};
pub use hasher::hash_const;

pub struct AST {
    pub source: String,
    pub user_code_start_line: u32,
    pub nodes: Vec<StmtNode>,
    pub hash_to_name_map: HashMap<NameHash, String>,
    pub static_classes: HashSet<NameHash>,
    pub class_map: HashMap<NameHash, Class>,
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        with_name_map(&self.hash_to_name_map, || {
            write!(f, "{:#?}\n{:#?}", self.class_map, self.nodes)
        })
    }
}

pub const MAIN_CLASS: NameHash = hash_const("main");

impl AST {
    pub fn new(source: String, user_code_start_line: u32) -> Self {
        let mut ast = Self {
            source,
            user_code_start_line,
            nodes: Vec::new(),
            class_map: HashMap::new(),
            hash_to_name_map: HashMap::new(),
            static_classes: HashSet::new(),
        };
        ast.hash("main"); // add into the hash map
        ast.class_map.insert(
            MAIN_CLASS,
            Class {
                line_info: LineInfo::default(),
                functions: HashMap::new(),
                public_vars: HashSet::new(),
                constructor: Constructor::default(),
                is_static: false,
            },
        );
        ast
    }

    pub fn add_class(
        &mut self,
        line: &LineInfo,
        class_name: &NameHash,
        class: Class,
        validator: &mut Validator,
    ) {
        if self.class_map.contains_key(class_name) {
            let class_name = &self.hash_to_name_map[class_name];
            compile_error(
                diagnostic(
                    line,
                    ErrorType::DuplicateName,
                    format!("class `{}` was already declared", class_name),
                    "duplicate name used",
                ),
                validator,
            )
        } else {
            self.class_map.insert(class_name.clone(), class);
        }
    }

    pub fn add_function(
        functions: &mut HashMap<NameHash, Function>,
        line: &LineInfo,
        fn_name: &NameHash,
        fn_real_name: &str,
        function: Function,
        validator: &mut Validator,
    ) {
        if functions.contains_key(fn_name) {
            compile_error(
                diagnostic(
                    line,
                    ErrorType::DuplicateName,
                    format!(
                        "function `{}` was already declared in this scope",
                        fn_real_name
                    ),
                    "duplicate name used",
                ),
                validator,
            )
        } else {
            functions.insert(fn_name.clone(), function);
        }
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
        let name_hash = hash_const(string);
        self.hash_to_name_map
            .insert(name_hash.clone(), string.to_string());
        name_hash
    }

    pub fn hash_with_this_keyword(&mut self, string: &str) -> NameHash {
        let string = "this.".to_string() + string;
        let name_hash = hash_const(string.as_str());

        self.hash_to_name_map.insert(name_hash.clone(), string);
        name_hash
    }

    pub fn hash_static_class(&mut self, string: &str) -> NameHash {
        let name_hash = self.hash(string);
        self.static_classes.insert(name_hash.clone());
        name_hash
    }

    pub fn format_val(&self, val: &Value, output: &mut String, env: &Env) {
        match val {
            Value::Number(n) => {
                if n.is_infinite() {
                    if n.is_sign_positive() {
                        output.push_str("Infinity");
                    } else {
                        output.push_str("-Infinity");
                    }
                } else if n.abs() > 1e20 {
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
        let (end_line, mut end_col) = span.end_pos().line_col();

        if end_line > start_line {
            let first_line_end_col = match span.lines().next() {
                Some(line) => line.chars().count() - 1,
                None => end_col,
            };
            end_col = first_line_end_col;
        }

        LineInfo {
            start_line: start_line as u32,
            start_col: start_col as u16,
            end_col: end_col as u16,
        }
    }
}
