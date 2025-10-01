use crate::data::NameHash;
use crate::data::diagnostic::Diagnostic;
use std::collections::{HashMap, HashSet};

pub struct Validator {
    pub validated_functions: HashMap<NameHash, HashSet<NameHash>>,
    pub errors: Vec<Diagnostic>,
}
