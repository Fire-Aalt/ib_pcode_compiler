use crate::ast::AST;
use crate::common::{num_op, str_op};
use crate::data::ast_nodes::{Expr, ExprNode, Operator, UnaryOp};
use crate::data::Value;
use crate::env::Env;
use std::collections::VecDeque;
use crate::data::diagnostic::{Diagnostic, ErrorType};

impl AST {
    pub fn eval_expr(&self, expr_node: &ExprNode, env: &mut Env) -> Result<Value, Diagnostic> {
        match &expr_node.expr {
            Expr::Ident(name) => match env.get(name) {
                Some(val) => Ok(val),
                None => expr_node.runtime_error(
                    ErrorType::Uninitialized,
                    format!("Variable {} was not initialized", name).as_str(),
                ),
            },
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
                    UnaryOp::Neg => -value,
                    UnaryOp::Not => !value,
                })
            }
            Expr::BinOp(left, op, right) => {
                let l = self.eval_expr(left, env)?;
                let r = self.eval_expr(right, env)?;

                Ok(match l {
                    Value::Number(l_num) => match r {
                        Value::Number(_) => num_op(l, op, r),
                        Value::Bool(_) => num_op(l, op, r),
                        Value::String(r_string) => Value::String(match op {
                            Operator::Add => l_num.to_string() + &*r_string,
                            _ => String::from("Nan"),
                        }),
                        _ => Value::String(String::from("Nan")),
                    },
                    Value::Bool(l_bool) => match r {
                        Value::Number(_) => num_op(l, op, r),
                        Value::Bool(r_bool) => Value::Bool(match op {
                            Operator::And => l_bool && r_bool,
                            Operator::Or => l_bool || r_bool,
                            _ => num_op(l, op, r).as_bool(),
                        }),
                        Value::String(r_string) => {
                            str_op(l.to_string().as_str(), op, r_string.as_str())
                        }
                        _ => Value::String(String::from("Nan")),
                    },
                    Value::String(l_string) => match r {
                        Value::Number(r_num) => Value::String(match op {
                            Operator::Add => l_string + &r_num.to_string(),
                            _ => String::from("Nan"),
                        }),
                        Value::Bool(_) => str_op(l_string.as_str(), op, r.to_string().as_str()),
                        Value::String(r_string) => str_op(l_string.as_str(), op, r_string.as_str()),
                        _ => Value::String(String::from("Nan")),
                    },
                    _ => Value::String(String::from("Nan")),
                })
            }
            Expr::Input(text) => {
                let text = self.eval_expr(text, env)?;
                Ok(self.exec_input(&text.to_string(), env))
            }
            Expr::Div(left, right) => {
                let left = self.eval_expr(left, env)?.as_num();
                let right = self.eval_expr(right, env)?.as_num();

                Ok(Value::Number((left as i64 / right as i64) as f64))
            }
            Expr::LocalMethodCall(fn_name, params) => {
                let class_name = &env.get_local_env().class_name.clone();
                
                let fn_def = self.get_function(class_name, fn_name).ok_or_else(|| {
                    expr_node.runtime_diagnostic(
                        ErrorType::Uninitialized,
                        format!("Undefined function in class {}", class_name).as_str())
                })?;

                let mut resolved_params = Vec::new();
                for param in params {
                    resolved_params.push(self.eval_expr(param, env)?);
                }

                let returned = self.exec_fn(fn_def, &resolved_params, env)?;
                match returned {
                    Some(val) => Ok(val),
                    None => expr_node.runtime_error(
                        ErrorType::NoReturn,
                        format!(
                            "No return found for function {} in class {}",
                            fn_name, class_name
                        )
                        .as_str(),
                    ),
                }
            }
            Expr::SubstringCall { expr, start, end } => {
                let val = self.eval_expr(expr, env)?;
                if let Value::String(s) = val {
                    let start = self.eval_expr(start, env)?.as_num() as usize;
                    let end = self.eval_expr(end, env)?.as_num() as usize;

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
                let index = self.eval_expr(index, env)?.as_num() as i64;

                match self.eval_expr(left, env)? {
                    Value::String(s) => {
                        let length = s.chars().count();

                        if index < 0 || index >= length as i64 {
                            return expr_node.runtime_error(
                                ErrorType::OutOfBounds,
                                format!("Index {} is out of bounds {}", index, length).as_str(),
                            );
                        }
                        Ok(Value::String(s.chars().nth(index as usize).unwrap().to_string()))
                    },
                    Value::Array(id) => {
                        let array = env.get_array_mut(&id);

                        if index < 0 || index >= array.len() as i64 {
                            return expr_node.runtime_error(
                                ErrorType::OutOfBounds,
                                format!("Index {} is out of bounds {}", index, array.len()).as_str(),
                            );
                        }

                        Ok(array[index as usize].clone())
                    }
                    _ => expr_node.runtime_error(ErrorType::InvalidType, "Invalid index expression"),
                }
            }
            Expr::ClassNew(class_name_hash, params) => {
                let class_def = self.get_class(class_name_hash).ok_or_else(|| {
                    expr_node.runtime_diagnostic(
                        ErrorType::Uninitialized,
                        "Undefined class",
                    )
                })?;
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
                    let class_name_hash = &env.get_class_name_hash(&id).clone();
                    let fn_def = self.get_function(class_name_hash, fn_name).ok_or_else(|| {
                        expr_node.runtime_diagnostic(
                            ErrorType::Uninitialized,
                            format!("Undefined function in class {}", class_name_hash).as_str(),
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
                        None => expr_node.runtime_error(
                            ErrorType::NoReturn,
                            format!(
                                "No return found for function {} in class {}",
                                fn_name, class_name_hash
                            )
                            .as_str(),
                        ),
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
