use crate::ast::AST;
use crate::common::{num_op, str_op};
use crate::data::ast_nodes::{Expr, ExprNode, Operand, UnaryOp};
use crate::data::Value;
use crate::env::Env;
use std::collections::VecDeque;
use crate::compiler::errors::{no_return_error, out_of_bounds_error, runtime_error, unsupported_operand_error};
use crate::data::diagnostic::{Diagnostic, ErrorType};

impl AST {
    pub fn eval_expr(&self, expr_node: &ExprNode, env: &mut Env) -> Result<Value, Diagnostic> {
        match &expr_node.expr {
            Expr::Ident(name) => Ok(env.get(name).unwrap()),
            Expr::Data(n) => Ok(n.clone()),
            Expr::Array(data) => {
                let mut array = VecDeque::new();

                for expr in data {
                    array.push_back(self.eval_expr(expr, env)?)
                }

                let id = env.create_array(array);
                Ok(Value::Array(id))
            }
            Expr::Unary(op, expr) => {
                let value = self.eval_expr(expr, env)?;
                Ok(match op {
                    UnaryOp::Neg => value.neg(&expr_node.line_info)?,
                    UnaryOp::Not => value.not(&expr_node.line_info)?,
                })
            }
            Expr::BinOp(left, op, right) => {
                let l = self.eval_expr(left, env)?;
                let r = self.eval_expr(right, env)?;

                let l_val = l.clone();
                let r_val = r.clone();

                // TODO: remove all of that and rely on conversions
                let res = match l {
                    Value::Number(l_num) => match r {
                        Value::Number(_) => Some(num_op(&expr_node.line_info, l, op, r)?),
                        Value::Bool(_) => Some(num_op(&expr_node.line_info, l, op, r)?),
                        Value::String(r_string) => match op {
                            Operand::Add => Some(Value::String(l_num.to_string() + &*r_string)),
                            _ => None,
                        },
                        _ => None,
                    },
                    Value::Bool(l_bool) => match r {
                        Value::Number(_) => Some(num_op(&expr_node.line_info, l, op, r)?),
                        Value::Bool(r_bool) => Some(match op {
                            Operand::And => Value::Bool(l_bool && r_bool),
                            Operand::Or => Value::Bool(l_bool || r_bool),
                            _ => Value::Bool(num_op(&expr_node.line_info, l, op, r)?.as_bool(&expr_node.line_info)?),
                        }),
                        Value::String(r_string) => {
                            Some(str_op(l.to_string().as_str(), op, r_string.as_str()))
                        }
                        _ => None,
                    },
                    Value::String(l_string) => match r {
                        Value::Number(r_num) => match op {
                            Operand::Add => Some(Value::String(l_string + &r_num.to_string())),
                            _ => None,
                        },
                        Value::Bool(_) => Some(str_op(l_string.as_str(), op, r.to_string().as_str())),
                        Value::String(r_string) => Some(str_op(l_string.as_str(), op, r_string.as_str())),
                        _ => None,
                    },
                    _ => None,
                };

                match res {
                    Some(val) => Ok(val),
                    None => runtime_error(&expr_node.line_info, unsupported_operand_error(&expr_node.line_info, l_val, op, r_val))
                }
            }
            Expr::Input(text) => {
                let text = self.eval_expr(text, env)?;
                Ok(self.exec_input(&text.to_string(), env))
            }
            Expr::Div(left, right) => {
                let left = self.eval_expr(left, env)?.as_num(&left.line_info)?;
                let right = self.eval_expr(right, env)?.as_num(&right.line_info)?;

                Ok(Value::Number((left as i64 / right as i64) as f64))
            }
            Expr::LocalMethodCall(fn_name, params) => {
                let class_name = &env.get_local_env().class_name.clone();
                let fn_def = self.get_function(class_name, fn_name).unwrap();

                let mut resolved_params = Vec::new();
                for param in params {
                    resolved_params.push(self.eval_expr(param, env)?);
                }

                // Local methods are already validated and no checks are needed
                let returned = self.exec_fn(fn_def, &resolved_params, env)?.unwrap_or(Value::Number(0.0));
                Ok(returned)
            }
            Expr::SubstringCall { expr, start, end } => {
                let val = self.eval_expr(expr, env)?;
                if let Value::String(s) = val {
                    let start = self.eval_expr(start, env)?.as_num(&start.line_info)? as usize;
                    let end = self.eval_expr(end, env)?.as_num(&end.line_info)? as usize;

                    Ok(Value::String(s[start..end].to_string()))
                } else {
                    expr_node.runtime_error(ErrorType::InvalidType, format!(".substring(start, end) used on {}. Only strings are supported", val).as_str())
                }
            }
            Expr::LengthCall(expr) => {
                let val = self.eval_expr(expr, env)?;
                match val {
                    Value::String(s) => Ok(Value::Number(s.len() as f64)),
                    Value::Array(id) => Ok(Value::Number(env.get_array(&id).len() as f64)),
                    _ => expr_node.runtime_error(ErrorType::InvalidType, format!(".length used on {}. Only strings and arrays are supported", val).as_str()),
                }
            }
            Expr::Index(left, index) => {
                let index = self.eval_expr(index, env)?.as_num(&index.line_info)? as i64;

                match self.eval_expr(left, env)? {
                    Value::String(s) => {
                        let length = s.chars().count();

                        if index < 0 || index >= length as i64 {
                            return runtime_error(&expr_node.line_info, out_of_bounds_error(index, length));
                        }
                        Ok(Value::String(s.chars().nth(index as usize).unwrap().to_string()))
                    },
                    Value::Array(id) => {
                        let array = env.get_array_mut(&id);

                        if index < 0 || index >= array.len() as i64 {
                            return runtime_error(&expr_node.line_info, out_of_bounds_error(index, array.len()));
                        }

                        Ok(array[index as usize].clone())
                    }
                    _ => expr_node.runtime_error(ErrorType::InvalidType, "Invalid index expression"),
                }
            }
            Expr::ClassNew(class_name_hash, params) => {
                let class_def = self.get_class(class_name_hash).unwrap();
                let id = env.create_local_env(class_name_hash.clone());

                env.push_local_env(id);
                // Define temp arg values
                for (i, param) in params.iter().enumerate() {
                    let arg_name_hash = &class_def.constructor.args[i];
                    let val = self.eval_expr(param, env)?;
                    env.define(arg_name_hash, val);
                }

                // Constructor
                for (name_hash, expr) in &class_def.constructor.constructors {
                    let val = self.eval_expr(expr, env)?;
                    env.define(name_hash, val);
                }

                // Undefine temp arg values
                for arg_name_hash in &class_def.constructor.args {
                    env.undefine(arg_name_hash);
                }
                env.pop_local_env();

                Ok(Value::Instance(id))
            }
            Expr::ClassMethodCall {
                expr,
                fn_name,
                params,
            } => {
                let val = self.eval_expr(expr, env)?;
                if let Value::Instance(id) = val {
                    let class_name = &env.get_class_name_hash(&id).clone();
                    let fn_def = self.get_function(class_name, fn_name).ok_or_else(|| {
                        expr_node.runtime_diagnostic(
                            ErrorType::Uninitialized,
                            format!("Undefined function in class {}", class_name).as_str(),
                        )
                    })?;

                    let mut resolved_params = Vec::new();
                    for param in params {
                        resolved_params.push(self.eval_expr(param, env)?);
                    }

                    env.push_local_env(id);
                    let returned = self.exec_fn(fn_def, &resolved_params, env)?;
                    env.pop_local_env();

                    return match returned {
                        Some(val) => Ok(val),
                        None => runtime_error(&expr_node.line_info, no_return_error(fn_name, class_name)),
                    };
                }
                expr_node.runtime_error(
                    ErrorType::InvalidType,
                    format!(
                        "Tried invoking a method {} not on an instance of a class: {}",
                        fn_name, val
                    )
                    .as_str(),
                )
            }
        }
    }
}
