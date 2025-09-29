use std::collections::{HashMap, HashSet};
use crate::data::diagnostic::Diagnostic;
use crate::data::NameHash;

pub struct Validator {
    pub validated_functions: HashMap<NameHash, HashSet<NameHash>>,
    pub errors: Vec<Diagnostic>,
}