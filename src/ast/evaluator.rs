use crate::ast::AST;
use crate::data::ast_nodes::{Expr, Function, Stmt};
use crate::data::Value;
use crate::env::{Env, EnvMode};
use std::io;
use std::io::Write;

mod exec_stmt;
mod eval_expr;

impl AST {
    pub fn traverse(&self, env: &mut Env) {
        for stmt in &self.statements {
            self.exec_stmt(stmt, env);
        }
    }

    fn exec_fn(&self, def: &Function, params: &[Value], env: &mut Env) -> Option<Value> {
        env.push_scope();
        self.define_method_params(def, params, env);
        let returned = self.exec_body(&def.body, env);
        env.pop_scope();

        returned
    }

    fn is_true(&self, cond: &Expr, env: &mut Env) -> bool {
        self.eval_expr(cond, env).as_bool()
    }

    fn define_method_params(&self, method_def: &Function, params: &[Value], env: &mut Env) {
        for (i, param) in params.iter().enumerate() {
            env.define(&method_def.args[i], param.clone());
        }
    }

    fn exec_body(&self, body: &Vec<Stmt>, env: &mut Env) -> Option<Value> {
        for stmt in body {
            if let Some(returned_val) = self.exec_stmt(stmt, env) {
                return Some(returned_val)
            }
        }
        None
    }

    fn exec_input(&self, ask_string: &str, env: &mut Env) -> Value {
        let mut input;

        match &mut env.mode {
            EnvMode::Release => {
                print!("{}: ", ask_string);
                io::stdout().flush().unwrap();

                input = String::new();
                io::stdin().read_line(&mut input).unwrap();
            }
            EnvMode::Test { mock_inputs, logs: _ } => {
                input = mock_inputs.pop_front().unwrap();
            }
        }
        let input = input.trim();

        match input.parse::<f64>() {
            Ok(f) => Value::Number(f),
            Err(_) => Value::String(input.to_string()),
        }
    }
}