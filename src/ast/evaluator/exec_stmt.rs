use crate::ast::AST;
use crate::data::ast_nodes::{AssignOperator, AssignTarget, Stmt, StmtNode};
use crate::data::Value;
use crate::env::{Env, EnvMode};
use std::cmp::max;
use crate::data::diagnostic::{Diagnostic, ErrorType};

impl AST {
    pub fn exec_stmt(&self, stmt_node: &StmtNode, env: &mut Env) -> Result<Option<Value>, Diagnostic> {
        match &stmt_node.stmt {
            Stmt::Assign(target, op, expr) => {
                let val = self.eval_expr(expr, env)?;
                self.exec_assign_stmt(stmt_node, target, op, val, env)?;
                Ok(None)
            }
            Stmt::Increment(target) => {
                self.exec_assign_stmt(stmt_node, target, &AssignOperator::AssignAdd, Value::Number(1.0), env)?;
                Ok(None)
            }
            Stmt::Decrement(target) => {
                self.exec_assign_stmt(stmt_node, target, &AssignOperator::AssignSubtract, Value::Number(1.0), env)?;
                Ok(None)
            }
            Stmt::If { cond, then_branch, elifs, else_branch } => {
                if self.is_true(cond, env)? {
                    return self.exec_body(then_branch, env);
                }

                for (elif_cond, elif_body) in elifs {
                    if self.is_true(elif_cond, env)? {
                        return self.exec_body(elif_body, env);
                    }
                }

                if let Some(body) = else_branch {
                    return self.exec_body(body, env);
                }
                Ok(None)
            }
            Stmt::While(cond, body) => {
                while self.is_true(cond, env)? {
                    if let Some(returned_val) = self.exec_body(body, env)? {
                        return Ok(Some(returned_val))
                    }
                }
                Ok(None)
            }
            Stmt::For(ident, start_num, end_num, body) => {
                let mut control = self.eval_expr(start_num, env)?;
                env.assign(ident, control.clone());

                while control <= self.eval_expr(end_num, env)? {
                    if let Some(returned_val) = self.exec_body(body, env)? {
                        return Ok(Some(returned_val))
                    }

                    control = env.get(ident).unwrap();
                    control = control + Value::Number(1.0);
                    env.assign(ident, control.clone());
                }
                Ok(None)
            }
            Stmt::Until(expr, body) => {
                while !self.is_true(expr, env)? {
                    if let Some(returned_val) = self.exec_body(body, env)? {
                        return Ok(Some(returned_val))
                    }
                }
                Ok(None)
            }
            Stmt::Input(ident) => {
                let input = self.exec_input(self.get_name(ident), env);
                env.assign(ident, input);
                Ok(None)
            }
            Stmt::Output(body) => {
                let mut output = String::new();

                for (i, expr) in body.iter().enumerate() {
                    if i > 0 {
                        output.push(' ');
                    }

                    let val = self.eval_expr(expr, env)?;
                    self.format_val(&val, &mut output, env);
                }

                match &mut env.mode {
                    EnvMode::Release => println!("{}", &output),
                    EnvMode::Test { mock_inputs: _, logs } => Env::record_log(logs, output),
                }
                Ok(None)
            }
            Stmt::Assert(expr, expected) => {
                assert_eq!(self.eval_expr(expr, env)?, self.eval_expr(expected, env)?);
                Ok(None)
            }
            Stmt::MethodReturn(expr) => Ok(Some(self.eval_expr(expr, env)?)),
            Stmt::Expr(expr) => {
                match self.eval_expr(expr, env) {
                    Err(e) => {
                        match e.error_type {
                            ErrorType::NoReturn => Ok(None),
                            _ => Err(e),
                        }
                    },
                    Ok(_) => Ok(None),
                }
            }
            Stmt::FunctionDeclaration(_) => Ok(None),
            Stmt::ClassDeclaration(_) => Ok(None),
            Stmt::EOI => Ok(None),
        }
    }

    fn exec_assign_stmt(&self, stmt: &StmtNode, target: &AssignTarget, op: &AssignOperator, val: Value, env: &mut Env) -> Result<Option<Value>, Diagnostic> {
        match target {
            AssignTarget::Ident(name) => {
                match op {
                    AssignOperator::Assign => env.assign(name, val),
                    AssignOperator::AssignAdd => env.assign(name, env.get(name).unwrap() + val),
                    AssignOperator::AssignSubtract => {
                        env.assign(name, env.get(name).unwrap() - val)
                    }
                    AssignOperator::AssignMultiply => {
                        env.assign(name, env.get(name).unwrap() * val)
                    }
                    AssignOperator::AssignDivide => env.assign(name, env.get(name).unwrap() / val),
                }
                Ok(None)
            }
            AssignTarget::Array(array_expr, index_expr) => {
                if let Value::Array(id) = self.eval_expr(array_expr, env)? {
                    let index = self.eval_expr(index_expr, env)?.as_num() as i64;
                    let array = env.get_array_mut(&id);

                    if index < 0 {
                        stmt.error(ErrorType::OutOfBounds, format!("Negative index: {}", index).as_str())?;
                    }
                    let index = index as usize;
                    while array.len() <= index {
                        array.resize(max(1, array.len()) * 2, Value::String("undefined".to_string()));
                    }

                    let res = match op {
                        AssignOperator::Assign => val,
                        AssignOperator::AssignAdd => array[index].clone() + val,
                        AssignOperator::AssignSubtract => array[index].clone() + val,
                        AssignOperator::AssignMultiply => array[index].clone() + val,
                        AssignOperator::AssignDivide => array[index].clone() + val,
                    };
                    array[index] = res;
                    Ok(None)
                } else {
                    stmt.error(ErrorType::InvalidType, "Invalid index expression")
                }
            }
        }
    }
}