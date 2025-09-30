mod validate_expr;
mod validate_stmt;

use crate::ast::{main_hash, AST};
use crate::data::ast_nodes::Function;
use crate::data::diagnostic::{Diagnostic, ErrorType, LineInfo};
use crate::data::{NameHash, Validator, Value};
use crate::env::Env;
use std::collections::HashMap;


impl AST {
    pub fn validate(&self, env: &mut Env) -> Vec<Diagnostic> {
        let mut validator = Validator { validated_functions: HashMap::new(), errors: Vec::new() };

        // Classes are encapsulated, so they can be checked fully first
        let _ = self.validate_class_definitions(env, &mut validator);

        // Main program execution flow check
        for stmt_node in &self.nodes {
            let _ = self.validate_stmt(stmt_node, env, &mut validator);
        }

        // Check unused methods in the main program
        let main = &main_hash();
        let id = env.create_local_env(main.clone());
        env.push_local_env(id);

        for (fn_name, function) in &self.class_map[main].functions {
            let _ = self.validate_fn_definition(main, fn_name, function, env, &mut validator);
        }
        env.pop_local_env();

        // Sort by first line, as some errors might be caught later
        validator.errors.sort_by(|left, right | {
            left.line_info.start_line.cmp(&right.line_info.start_line)
        });
        validator.errors
    }

    fn validate_class_definitions(&self, env: &mut Env, validator: &mut Validator) -> Result<(), Diagnostic> {
        for (class_name, class) in &self.class_map {
            if class_name == main_hash() {
                continue
            }

            let id = env.create_local_env(class_name.clone());

            env.push_local_env(id);
            for arg in &class.constructor.args {
                env.define(arg, Value::Number(0.0))
            }

            for (arg, expr_node) in &class.constructor.constructors {
                let _ = self.validate_expr(expr_node, env, validator);
                env.define(arg, Value::Number(0.0))
            }

            for (fn_name, function) in &class.functions {
                let _ = self.validate_fn_definition(class_name, fn_name, function, env, validator);
            }

            env.pop_local_env();
        }
        Ok(())
    }

    fn validate_fn_definition(&self, class_name: &NameHash, fn_name: &NameHash, function: &Function, env: &mut Env, validator: &mut Validator) -> Result<(), Diagnostic> {
        let entry = validator.validated_functions.entry(class_name.clone()).or_default();
        if !entry.contains(fn_name) {
            env.push_scope();
            for arg in &function.args {
                env.define(arg, Value::Number(0.0))
            }

            for stmt_node in &function.body {
                let _ = self.validate_stmt(stmt_node, env, validator);
            }
            env.pop_scope();

            validator.validated_functions.get_mut(class_name).unwrap().insert(fn_name.clone());
        }
        Ok(())
    }

    pub fn validate_fn_call(&self, line_info: &LineInfo, class_name: &NameHash, fn_name: &NameHash, validator: &mut Validator) -> Result<(), Diagnostic> {
        self.get_function(class_name, fn_name).ok_or_else(|| line_info.compile_error(
            ErrorType::Uninitialized,
            format!("cannot find function `{}` in this scope", fn_name).as_str(),
            "not found in this scope",
            validator
        )).err().unwrap_or(Ok(()))
    }

    pub fn validate_class_call(&self, line_info: &LineInfo, class_name: &NameHash, validator: &mut Validator) -> Result<(), Diagnostic> {
        self.get_class(class_name).ok_or_else(|| line_info.compile_error(
            ErrorType::Uninitialized,
            format!("cannot find class `{}`", class_name).as_str(),
            "class is not defined",
            validator
        )).err().unwrap_or(Ok(()))
    }
}