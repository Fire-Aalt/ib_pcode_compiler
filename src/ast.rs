use crate::ast_nodes::{Class, Function, Stmt};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Default)]
pub struct AST {
    pub statements: Vec<Stmt>,
    pub function_map: HashMap<NameHash, Function>,
    pub class_map: HashMap<NameHash, Class>,
    pub hash_to_name_map: HashMap<NameHash, String>
}

#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
pub struct NameHash {
    pub hash: u64,
    pub this_keyword: bool
}

impl AST {
    pub fn get_fn_definition(&self, class_name: &NameHash, fn_name: &NameHash) -> &Function {
        self.class_map.get(class_name).unwrap().functions.get(fn_name).unwrap()
    }

    pub fn hash(&mut self, string: &str) -> NameHash {
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
        
        self.hash_to_name_map.insert(name_hash.clone(), string.to_string());
        name_hash
    }
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.statements)
    }
}
