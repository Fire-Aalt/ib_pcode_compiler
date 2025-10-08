mod validate_expr;
mod validate_stmt;

use crate::ast::{AST, MAIN_CLASS};
use crate::compiler::errors::{compile_error, diagnostic, invalid_number_of_params_error};
use crate::data::ast_nodes::{Class, ExprNode, Function};
use crate::data::diagnostic::{ErrorType, LineInfo};
use crate::data::{NameHash, Validator, Value};
use crate::env::Env;
use std::cmp::Ordering;
use std::collections::HashMap;

impl AST {
    pub fn validate(&self, env: &mut Env) -> Validator {
        let mut validator = Validator {
            validated_functions: HashMap::new(),
            errors: Vec::new(),
            added_errors: 0,
        };

        // Classes are encapsulated, so they can be checked fully first
        self.validate_class_definitions(env, &mut validator);

        // Main program execution flow check
        for stmt_node in &self.nodes {
            self.validate_stmt(stmt_node, env, &mut validator);
        }

        // Check unused methods in the main program
        let id = env.create_local_env(MAIN_CLASS);
        env.push_local_env(id);

        for (fn_name, function) in &self.class_map[&MAIN_CLASS].functions {
            self.validate_fn_definition(&MAIN_CLASS, fn_name, function, env, &mut validator);
        }
        env.pop_local_env();

        // Sort by first line, as some errors might be caught later
        validator.errors.sort_by(|left, right| {
            let mut ord = left.line_info.start_line.cmp(&right.line_info.start_line);

            if ord == Ordering::Equal {
                ord = left.line_info.start_col.cmp(&right.line_info.start_col);
            };
            ord
        });
        validator
    }

    fn validate_class_definitions(&self, env: &mut Env, validator: &mut Validator) {
        for (class_name, class) in &self.class_map {
            if class_name == MAIN_CLASS {
                continue;
            }

            let id = env.create_local_env(class_name.clone());
            env.push_local_env(id);

            if class.is_static {
                env.static_envs.insert(class_name.clone(), id);

                if !class.constructor.args.is_empty() {
                    compile_error(
                        diagnostic(
                            &class.constructor.line_info,
                            ErrorType::Unsupported,
                            format!(
                                "constructor parameter(s) found for class `{}`. Static classes cannot have constructor parameters",
                                class_name
                            ),
                            "invalid constructor parameter(s)",
                        ),
                        validator,
                    );
                }
            } else {
                for arg in &class.constructor.args {
                    env.define(arg, Value::Number(0.0))
                }
            }

            for (arg, expr_node) in &class.constructor.constructors {
                self.validate_expr(expr_node, env, validator);
                env.define(arg, Value::Number(0.0))
            }

            for (fn_name, function) in &class.functions {
                self.validate_fn_definition(class_name, fn_name, function, env, validator);
            }

            env.pop_local_env();
        }
    }

    fn validate_fn_definition(
        &self,
        class_name: &NameHash,
        fn_name: &NameHash,
        function: &Function,
        env: &mut Env,
        validator: &mut Validator,
    ) {
        let entry = validator
            .validated_functions
            .entry(class_name.clone())
            .or_default();
        if !entry.contains(fn_name) {
            env.push_scope();
            for arg in &function.args {
                env.define(arg, Value::Number(0.0))
            }

            for stmt_node in &function.body {
                self.validate_stmt(stmt_node, env, validator);
            }
            env.pop_scope();

            validator
                .validated_functions
                .get_mut(class_name)
                .unwrap()
                .insert(fn_name.clone());
        }
    }

    pub fn validate_class_get(
        &self,
        line_info: &LineInfo,
        class_name: &NameHash,
        validator: &mut Validator,
    ) -> Option<&Class> {
        match self.get_class(class_name) {
            Some(class_def) => Some(class_def),
            None => {
                compile_error(
                    diagnostic(
                        line_info,
                        ErrorType::Uninitialized,
                        format!("cannot find class `{}`", class_name),
                        "class is not defined",
                    ),
                    validator,
                );
                None
            }
        }
    }

    pub fn valid_number_of_args(
        line_info: &LineInfo,
        params: &[ExprNode],
        assert: fn(usize) -> bool,
        expected: &'static &str,
        validator: &mut Validator,
    ) -> bool {
        if assert(params.len()) {
            return true;
        }
        compile_error(
            invalid_number_of_params_error(line_info, params.len(), expected.to_string()),
            validator,
        );
        false
    }
}
