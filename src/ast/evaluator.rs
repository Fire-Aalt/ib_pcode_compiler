use crate::ast::AST;
use crate::data::ast_nodes::{ExprNode, Function, Operand, StmtNode};
use crate::data::diagnostic::Diagnostic;
use crate::data::Value;
use crate::env::{Env, EnvMode};
use std::io;
use std::io::Write;

mod exec_stmt;
mod eval_expr;

impl AST {
    pub fn traverse(&self, env: &mut Env) -> Result<(), Diagnostic> {
        for stmt_node in &self.nodes {
            self.exec_stmt(stmt_node, env)?;
        }
        Ok(())
    }

    fn exec_fn(&self, def: &Function, params: &[Value], env: &mut Env) -> Result<Option<Value>, Diagnostic> {
        env.push_scope();
        self.define_method_params(def, params, env);
        let returned = self.exec_body(&def.body, env)?;
        env.pop_scope();

        Ok(returned)
    }

    fn is_true(&self, cond: &ExprNode, env: &mut Env) -> Result<bool, Diagnostic> {
        self.eval_expr(cond, env)?.as_bool(&cond.line_info)
    }

    fn define_method_params(&self, method_def: &Function, params: &[Value], env: &mut Env) {
        for (i, param) in params.iter().enumerate() {
            env.define(&method_def.args[i], param.clone());
        }
    }

    fn exec_body(&self, body: &Vec<StmtNode>, env: &mut Env) -> Result<Option<Value>, Diagnostic> {
        env.push_scope();
        for stmt in body {
            if let Some(returned_val) = self.exec_stmt(stmt, env)? {
                env.pop_scope();
                return Ok(Some(returned_val))
            }
        }
        env.pop_scope();
        Ok(None)
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

    fn and_or_op(&self, l_val: &Value, op: &Operand, r_expr: &ExprNode, env: &mut Env) -> Result<Option<Value>, Diagnostic> {
        // On demand evaluation: `&&` fails if just first check fails, `||` succeeds if first check succeeds
        match op {
            Operand::And => Ok(Some(Value::Bool(l_val.as_bool_unsafe() && r_expr.eval_as_bool_unsafe(self, env)?))),
            Operand::Or => Ok(Some(Value::Bool(l_val.as_bool_unsafe() || r_expr.eval_as_bool_unsafe(self, env)?))),
            _ => Ok(None),
        }
    }

    fn num_operations(&self, l: &Value, op: &Operand, r: &Value) -> Option<Value> {
        match r {
            Value::Number(_) => self.num_op(l, op, r),
            Value::Bool(_) => self.num_op(l, op, r),
            Value::String(_) => self.str_op(l, op, r),
            _ => None,
        }
    }

    fn num_op(&self, l_val: &Value, op: &Operand, r_val: &Value) -> Option<Value> {
        let l= l_val.as_num_unsafe();
        let r = r_val.as_num_unsafe();

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
            Operand::Equal => Value::Bool(l == r),
            Operand::NotEqual => Value::Bool(l != r),
            Operand::And => unreachable!(),
            Operand::Or => unreachable!(),
        };
        Some(res)
    }

    fn str_op(&self, l_val: &Value, op: &Operand, r_val: &Value) -> Option<Value> {
        let l= l_val.as_string();
        let r = r_val.as_string();

        let res = match op {
            Operand::Add => Some(Value::String(l + r.as_str())),
            Operand::Greater => Some(Value::Bool(l > r)),
            Operand::Less => Some(Value::Bool(l < r)),
            Operand::GreaterEqual => Some(Value::Bool(l >= r)),
            Operand::LessEqual => Some(Value::Bool(l <= r)),
            Operand::Equal => Some(Value::Bool(l == r)),
            Operand::NotEqual => Some(Value::Bool(l != r)),
            _ => None,
        };
        res
    }
}