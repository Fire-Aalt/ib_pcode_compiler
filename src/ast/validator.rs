mod valid_expr;
mod valid_stmt;

use std::collections::HashSet;
use crate::ast::{main_hash, AST};
use crate::data::ast_nodes::Function;
use crate::data::diagnostic::{Diagnostic, ErrorType, LineInfo};
use crate::data::{NameHash, Value};
use crate::env::Env;

pub struct Validator {
    pub validated_functions: HashSet<(NameHash, NameHash)>
}

impl AST {
    pub fn validate(&self, env: &mut Env) -> Result<(), Diagnostic> {
        let mut validator = Validator { validated_functions: HashSet::new() };
        self.valid_class_definitions(env, &mut validator)?;

        for stmt_node in &self.nodes {
            self.valid_stmt(stmt_node, env, &mut validator)?;
        }
        Ok(())
    }

    fn valid_class_definitions(&self, env: &mut Env, validator: &mut Validator) -> Result<(), Diagnostic> {
        for (class_name, class) in &self.class_map.clone() {
            if class_name == main_hash() {
                return Ok(())
            }

            let id = env.create_local_env(class_name.clone());

            env.push_local_env(id);
            for arg in &class.constructor.args {
                env.define(arg, Value::Number(0.0))
            }

            for (arg, expr_node) in &class.constructor.constructors {
                self.valid_expr(expr_node, env, validator)?;
                env.define(arg, Value::Number(0.0))
            }

            for (fn_name, function) in &class.functions {
                self.valid_fn_definition(class_name, fn_name, function, env, validator)?;
            }

            env.pop_local_env();
        }
        Ok(())
    }

    fn valid_fn_definition(&self, class_name: &NameHash, fn_name: &NameHash, function: &Function, env: &mut Env, validator: &mut Validator) -> Result<(), Diagnostic> {
        if !validator.validated_functions.contains(&(class_name.clone(), fn_name.clone())) {
            env.push_scope();
            for arg in &function.args {
                env.define(arg, Value::Number(0.0))
            }

            for stmt_node in &function.body {
                self.valid_stmt(stmt_node, env, validator)?;
            }
            env.pop_scope();

            validator.validated_functions.insert((class_name.clone(), fn_name.clone()));
        }
        Ok(())
    }

    pub fn valid_fn_call(&self, line_info: &LineInfo, class_name: &NameHash, fn_name: &NameHash) -> Result<(), Diagnostic> {
        self.get_function(class_name, fn_name).ok_or_else(|| line_info.valid_error(
            ErrorType::Uninitialized,
            format!("Undefined function in class {}", class_name).as_str(),
        )).err().unwrap_or(Ok(()))
    }

    pub fn valid_class_call(&self, line_info: &LineInfo, class_name: &NameHash) -> Result<(), Diagnostic> {
        self.get_class(class_name).ok_or_else(|| line_info.valid_error(
            ErrorType::Uninitialized,
            format!("Undefined function in class {}", class_name).as_str(),
        )).err().unwrap_or(Ok(()))
    }
}