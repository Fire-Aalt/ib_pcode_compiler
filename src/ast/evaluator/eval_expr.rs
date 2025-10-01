use crate::ast::AST;
use crate::compiler::errors::{
    diagnostic, invalid_type_call_error, no_return_error, out_of_bounds_error,
    unsupported_operand_error,
};
use crate::data::Value;
use crate::data::ast_nodes::{Expr, ExprNode, UnaryOp};
use crate::data::diagnostic::{Diagnostic, ErrorType};
use crate::env::Env;
use std::collections::VecDeque;
use rand::Rng;

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
                Ok(Value::ArrayId(id))
            }
            Expr::Unary(op, expr) => {
                let value = self.eval_expr(expr, env)?;
                Ok(match op {
                    UnaryOp::Neg => value.neg(&expr_node.line_info)?,
                    UnaryOp::Not => value.not(&expr_node.line_info)?,
                })
            }
            Expr::BinOp(left, op, right) => {
                let l = &self.eval_expr(left, env)?;

                match self.and_or_op(l, op, right, env)? {
                    Some(val) => Ok(val),
                    None => {
                        let r = &self.eval_expr(right, env)?;

                        let res = match l {
                            Value::Number(_) => self.num_operations(l, op, r),
                            Value::Bool(_) => self.num_operations(l, op, r),
                            Value::String(_) => match r {
                                Value::Number(_) => self.str_op(l, op, r),
                                Value::Bool(_) => self.str_op(l, op, r),
                                Value::String(_) => self.str_op(l, op, r),
                                _ => None,
                            },
                            _ => None,
                        };

                        match res {
                            Some(val) => Ok(val),
                            None => Err(unsupported_operand_error(&expr_node.line_info, l, op, r)),
                        }
                    }
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
            Expr::MathRandom => {
                let mut rng = rand::rng();
                Ok(Value::Number(rng.random_range(0.0..1.0)))
            }
            Expr::LocalMethodCall(fn_name, params) => {
                let class_name = &env.get_local_env().class_name.clone();
                let fn_def = self.get_function(class_name, fn_name).unwrap();

                let mut resolved_params = Vec::new();
                for param in params {
                    resolved_params.push(self.eval_expr(param, env)?);
                }

                // Local methods are already validated and no checks are needed
                let returned = self
                    .exec_fn(fn_def, &resolved_params, env)?
                    .unwrap_or(Value::Number(0.0));
                Ok(returned)
            }
            Expr::SubstringCall { expr, start, end } => {
                let val = &self.eval_expr(expr, env)?;
                if let Value::String(s) = val {
                    let start = self.eval_expr(start, env)?.as_num(&start.line_info)? as usize;
                    let end = self.eval_expr(end, env)?.as_num(&end.line_info)? as usize;

                    Ok(Value::String(s[start..end].to_string()))
                } else {
                    Err(invalid_type_call_error(
                        &expr.line_info,
                        "`.substring(start, end)`",
                        val,
                        "strings",
                    ))
                }
            }
            Expr::LengthCall(expr) => {
                let val = &self.eval_expr(expr, env)?;
                match val {
                    Value::String(s) => Ok(Value::Number(s.len() as f64)),
                    Value::ArrayId(id) => Ok(Value::Number(env.get_array(&id).len() as f64)),
                    _ => Err(invalid_type_call_error(
                        &expr.line_info,
                        "`.length`",
                        val,
                        "strings and arrays",
                    )),
                }
            }
            Expr::Index(left, index) => {
                let index = self.eval_expr(index, env)?.as_num(&index.line_info)? as i64;

                let val = &self.eval_expr(left, env)?;
                match val {
                    Value::String(s) => {
                        let length = s.chars().count();

                        if index < 0 || index >= length as i64 {
                            return Err(out_of_bounds_error(&expr_node.line_info, index, length));
                        }
                        Ok(Value::String(
                            s.chars().nth(index as usize).unwrap().to_string(),
                        ))
                    }
                    Value::ArrayId(id) => {
                        let array = env.get_array_mut(id);

                        if index < 0 || index >= array.len() as i64 {
                            return Err(out_of_bounds_error(
                                &expr_node.line_info,
                                index,
                                array.len(),
                            ));
                        }

                        Ok(array[index as usize].clone())
                    }
                    _ => Err(invalid_type_call_error(
                        &expr_node.line_info,
                        "index expression",
                        val,
                        "strings and arrays",
                    )),
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

                Ok(Value::InstanceId(id))
            }
            Expr::ClassMethodCall {
                expr,
                fn_name,
                params,
            } => {
                let val = self.eval_expr(expr, env)?;
                if let Value::InstanceId(id) = val {
                    let class_name = &env.get_class_name_hash(&id).clone();
                    let fn_def = self.get_function(class_name, fn_name).ok_or_else(|| {
                        diagnostic(
                            &expr_node.line_info,
                            ErrorType::Uninitialized,
                            format!("undefined function `{}` in class `{}`", fn_name, class_name),
                            "uninitialized function",
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
                        None => Err(no_return_error(&expr_node.line_info, fn_name, class_name)),
                    };
                }
                Err(diagnostic(
                    &expr_node.line_info,
                    ErrorType::InvalidType,
                    format!(
                        "tried invoking a method `{}` not on an instance of a class: `{}`",
                        fn_name, val
                    ),
                    "",
                ))
            }
        }
    }
}
