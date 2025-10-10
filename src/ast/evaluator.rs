use crate::ast::AST;
use crate::data::Value;
use crate::data::ast_nodes::{ExprNode, Function, Operand, StmtNode};
use crate::data::diagnostic::{Diagnostic, LineInfo};
use crate::env::{Env, EnvMode};
use std::io;
use std::io::Write;

mod eval_expr;
mod exec_stmt;

impl AST {
    pub fn traverse(&self, env: &mut Env) -> Result<(), Diagnostic> {
        for name in &self.static_classes {
            let class_def = &self.class_map[name];

            let id = env.create_local_env(name.clone());
            env.static_envs.insert(name.clone(), id);
            env.push_local_env(id);

            // Constructor
            for (name_hash, expr) in &class_def.constructor.constructors {
                let val = self.eval_expr(expr, env)?;
                env.define(name_hash, val);
            }

            env.pop_local_env();
        }

        for stmt_node in &self.nodes {
            self.exec_stmt(stmt_node, env)?;
        }
        Ok(())
    }

    fn exec_fn(
        &self,
        def: &Function,
        params: &[Value],
        env: &mut Env,
    ) -> Result<Option<Value>, Diagnostic> {
        env.push_scope();
        Self::define_method_params(def, params, env);
        let returned = self.exec_body(&def.body, env)?;
        env.pop_scope();

        Ok(returned)
    }

    fn is_true(&self, cond: &ExprNode, env: &mut Env) -> Result<bool, Diagnostic> {
        self.eval_expr(cond, env)?.as_bool(&cond.line_info)
    }

    fn define_method_params(method_def: &Function, params: &[Value], env: &mut Env) {
        for (i, param) in params.iter().enumerate() {
            env.define(&method_def.args[i], param.clone());
        }
    }

    fn exec_body(&self, body: &Vec<StmtNode>, env: &mut Env) -> Result<Option<Value>, Diagnostic> {
        env.push_scope();
        for stmt in body {
            if let Some(returned_val) = self.exec_stmt(stmt, env)? {
                env.pop_scope();
                return Ok(Some(returned_val));
            }
        }
        env.pop_scope();
        Ok(None)
    }

    fn exec_input(ask_string: &str, env: &mut Env) -> Value {
        let mut input;

        match &mut env.mode {
            EnvMode::Release => {
                print!("{}: ", ask_string);
                io::stdout().flush().unwrap();

                input = String::new();
                io::stdin().read_line(&mut input).unwrap();
            }
            EnvMode::Test {
                mock_inputs,
                logs: _,
            } => {
                input = mock_inputs.pop_front().unwrap();
            }
        }
        let input = input.trim();

        match input.parse::<f64>() {
            Ok(f) => Value::Number(f),
            Err(_) => Value::String(input.to_string()),
        }
    }

    fn and_or_operations(
        &self,
        l_val_line: &LineInfo,
        l_val: &Value,
        op: &Operand,
        r_expr: &ExprNode,
        env: &mut Env,
    ) -> Result<Option<Value>, Diagnostic> {
        // On demand evaluation: `&&` fails if just first check fails, `||` succeeds if first check succeeds
        let res = match op {
            Operand::And => {
                Value::Bool(l_val.as_bool(l_val_line)? && r_expr.eval_as_bool(self, env)?)
            }
            Operand::Or => {
                Value::Bool(l_val.as_bool(l_val_line)? || r_expr.eval_as_bool(self, env)?)
            }
            _ => return Ok(None),
        };
        Ok(Some(res))
    }

    fn equality_operations(l_val: &Value, op: &Operand, r_val: &Value) -> Option<Value> {
        let res = match op {
            Operand::Equal => Value::Bool(l_val == r_val),
            Operand::NotEqual => Value::Bool(l_val != r_val),
            _ => return None,
        };
        Some(res)
    }

    fn num_operations(l: &Value, op: &Operand, r: &Value) -> Option<Value> {
        match r {
            Value::Number(_) => Self::num_op(l, op, r),
            Value::Bool(_) => Self::num_op(l, op, r),
            Value::String(_) => Self::str_op(l, op, r),
            _ => None,
        }
    }

    fn num_op(l_val: &Value, op: &Operand, r_val: &Value) -> Option<Value> {
        let l = unsafe { l_val.as_num_unchecked() };
        let r = unsafe { r_val.as_num_unchecked() };

        let res = match op {
            Operand::Add => Value::Number(l + r),
            Operand::Subtract => Value::Number(l - r),
            Operand::Multiply => Value::Number(l * r),
            Operand::Divide => Value::Number(l / r),
            Operand::IntDivide => Value::Number((l as i64 / r as i64) as f64),
            Operand::Power => Value::Number(l.powf(r)),
            Operand::Modulo => Value::Number(l % r),
            Operand::Greater => Value::Bool(l > r),
            Operand::Less => Value::Bool(l < r),
            Operand::GreaterEqual => Value::Bool(l >= r),
            Operand::LessEqual => Value::Bool(l <= r),
            Operand::And | Operand::Or | Operand::Equal | Operand::NotEqual => unreachable!(),
        };
        Some(res)
    }

    fn str_op(l_val: &Value, op: &Operand, r_val: &Value) -> Option<Value> {
        let l = l_val.as_string();
        let r = r_val.as_string();

        let res = match op {
            Operand::Add => Value::String(l + r.as_str()),
            Operand::Greater => Value::Bool(l > r),
            Operand::Less => Value::Bool(l < r),
            Operand::GreaterEqual => Value::Bool(l >= r),
            Operand::LessEqual => Value::Bool(l <= r),
            Operand::Equal => Value::Bool(l == r),
            Operand::NotEqual => Value::Bool(l != r),
            _ => return None,
        };
        Some(res)
    }
}
