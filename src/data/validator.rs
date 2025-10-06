use crate::data::NameHash;
use crate::data::diagnostic::{Diagnostic, ErrorType};
use std::collections::{HashMap, HashSet};

pub struct Validator {
    pub validated_functions: HashMap<NameHash, HashSet<NameHash>>,
    pub errors: Vec<Diagnostic>,
    pub added_errors: u32,
}

impl Validator {
    pub fn start_record(&mut self) {
        self.added_errors = 0;
    }

    pub fn is_last_recorded_expr_error(&self, error_type: ErrorType) -> bool {
        if self.added_errors == 0 {
            return false;
        }

        match self.errors.last() {
            Some(last_error) => {
                if last_error.error_type == error_type {
                    return true;
                }
                false
            }
            None => false,
        }
    }
}
