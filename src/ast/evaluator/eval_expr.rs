use crate::ast::AST;
use crate::compiler::errors::{
    diagnostic, invalid_number_of_params_error, invalid_type_call_error, no_public_var_error,
    no_return_error, out_of_bounds_error, undefined_fn_in_class_error, unsupported_operand_error,
};
use crate::data::Value;
use crate::data::ast_nodes::{Expr, ExprNode, NativeMethod, UnaryOp};
use crate::data::diagnostic::{Diagnostic, ErrorType};
use crate::env::Env;
use rand::Rng;
use std::collections::VecDeque;

impl AST {
    pub fn eval_expr(&self, expr_node: &ExprNode, env: &mut Env) -> Result<Value, Diagnostic> {
        let line = &expr_node.line_info;
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
                    UnaryOp::Neg => value.neg(line)?,
                    UnaryOp::Not => value.not(line)?,
                })
            }
            Expr::BinOp(left, op, right) => {
                let left_val = self.eval_expr(left, env)?;

                // First try short-circuit AND/OR handling
                if let Some(v) =
                    self.and_or_operations(&left.line_info, &left_val, op, right, env)?
                {
                    return Ok(v);
                }
                let right_val = self.eval_expr(right, env)?;

                // Prioritize string specific operations
                let result = if (matches!(left_val, Value::String(_))
                    && matches!(
                        right_val,
                        Value::String(_) | Value::Number(_) | Value::Bool(_)
                    ))
                    || (matches!(
                        left_val,
                        Value::String(_) | Value::Number(_) | Value::Bool(_)
                    ) && matches!(right_val, Value::String(_)))
                {
                    Self::str_op(&left_val, op, &right_val)
                } else {
                    // Anytype equality operations
                    if let Some(v) = Self::equality_operations(&left_val, op, &right_val) {
                        return Ok(v);
                    }

                    // Number operations
                    match (&left_val, &right_val) {
                        (Value::Number(_), _) | (Value::Bool(_), _) => {
                            Self::num_operations(&left_val, op, &right_val)
                        }
                        _ => None,
                    }
                };

                match result {
                    Some(v) => Ok(v),
                    None => Err(unsupported_operand_error(line, &left_val, op, &right_val)),
                }
            }
            Expr::NativeMethodCall(native_method, target, fn_line, params) => match native_method {
                NativeMethod::Div => {
                    let left = &params[0];
                    let right = &params[1];

                    let left = self.eval_expr(left, env)?.as_num(&left.line_info)?;
                    let right = self.eval_expr(right, env)?.as_num(&right.line_info)?;

                    Ok(Value::Number((left as i64 / right as i64) as f64))
                }
                NativeMethod::Input => {
                    let text = if params.len() == 1 {
                        self.eval_expr(&params[0], env)?
                    } else {
                        Value::String("".into())
                    };
                    Ok(Self::exec_input(&text.fmt(), env))
                }
                NativeMethod::MathRandom => {
                    let mut rng = rand::rng();
                    Ok(Value::Number(rng.random_range(0.0..1.0)))
                }
                NativeMethod::SubstringCall => {
                    let expr = target.as_ref().unwrap();
                    let start = &params[0];
                    let end = &params[1];

                    let val = &self.eval_expr(expr, env)?;
                    if let Value::String(s) = val {
                        let start = self.eval_expr(start, env)?.as_num(&start.line_info)? as usize;
                        let end = self.eval_expr(end, env)?.as_num(&end.line_info)? as usize;

                        Ok(Value::String(s[start..end].to_string()))
                    } else {
                        Err(invalid_type_call_error(
                            fn_line,
                            "`.substring(start, end)`",
                            val,
                            "strings",
                            "method",
                        ))
                    }
                }
                NativeMethod::LengthCall => {
                    let expr = target.as_ref().unwrap();
                    let val = &self.eval_expr(expr, env)?;
                    match val {
                        Value::String(s) => Ok(Value::Number(s.len() as f64)),
                        Value::ArrayId(id) => Ok(Value::Number(env.get_array(id).len() as f64)),
                        _ => Err(invalid_type_call_error(
                            &expr.line_info,
                            "`.length`",
                            val,
                            "strings and arrays",
                            "variable does not exist",
                        )),
                    }
                }
            },
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
            Expr::Index(left, index) => {
                let index = self.eval_expr(index, env)?.as_num(&index.line_info)? as i64;

                let val = &self.eval_expr(left, env)?;
                match val {
                    Value::String(s) => {
                        let length = s.chars().count();

                        if index < 0 || index >= length as i64 {
                            return Err(out_of_bounds_error(line, index, length));
                        }
                        Ok(Value::String(
                            s.chars().nth(index as usize).unwrap().to_string(),
                        ))
                    }
                    Value::ArrayId(id) => {
                        let array = env.get_array_mut(id);

                        if index < 0 || index >= array.len() as i64 {
                            return Err(out_of_bounds_error(line, index, array.len()));
                        }

                        Ok(array[index as usize].clone())
                    }
                    _ => Err(invalid_type_call_error(
                        line,
                        "index expression",
                        val,
                        "strings and arrays",
                        "invalid index expression",
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
                fn_line,
                fn_name,
                params,
            } => {
                let val = self.eval_expr(expr, env)?;

                if let Value::InstanceId(id) = val {
                    let class_name = &env.get_class_name_hash(&id).clone();
                    let fn_def = self
                        .get_function(class_name, fn_name)
                        .ok_or_else(|| undefined_fn_in_class_error(fn_line, class_name, fn_name))?;

                    if params.len() != fn_def.args.len() {
                        return Err(invalid_number_of_params_error(
                            fn_line,
                            params.len(),
                            fn_def.args.len().to_string(),
                        ));
                    }

                    let mut resolved_params = Vec::new();
                    for param in params {
                        resolved_params.push(self.eval_expr(param, env)?);
                    }

                    env.push_local_env(id);
                    let returned = self.exec_fn(fn_def, &resolved_params, env)?;
                    env.pop_local_env();

                    return match returned {
                        Some(val) => Ok(val),
                        None => Err(no_return_error(fn_line, fn_name, class_name)),
                    };
                }
                Err(diagnostic(
                    line,
                    ErrorType::InvalidType,
                    format!(
                        "tried invoking a method `{}` not on an instance of a class: `{}`",
                        fn_name, val
                    ),
                    "",
                ))
            }
            Expr::ClassGetVar(expr, var_line, var_name) => {
                let val = self.eval_expr(expr, env)?;

                match val {
                    Value::InstanceId(id) => {
                        let class_name = &env.get_class_name_hash(&id).clone();
                        let class_def = self.get_class(class_name).unwrap();

                        if !class_def.public_vars.contains(var_name) {
                            return Err(no_public_var_error(var_line, var_name, class_name));
                        }

                        env.push_local_env(id);
                        let returned = env.get(var_name).unwrap();
                        env.pop_local_env();

                        Ok(returned)
                    }
                    _ => Err(diagnostic(
                        line,
                        ErrorType::InvalidType,
                        format!(
                            "tried accessing a variable `{}` not on an instance of a class: `{}`",
                            var_name, val
                        ),
                        "",
                    )),
                }
            }
            Expr::StaticMethodCall(_, class_name, fn_name, params) => {
                let fn_def = self.get_function(class_name, fn_name).unwrap();

                let mut resolved_params = Vec::new();
                for param in params {
                    resolved_params.push(self.eval_expr(param, env)?);
                }

                let id = env.static_envs[class_name];
                env.push_local_env(id);
                // Static methods are already validated and no checks are needed
                let returned = self
                    .exec_fn(fn_def, &resolved_params, env)?
                    .unwrap_or(Value::Number(0.0));
                env.pop_local_env();
                Ok(returned)
            }
            Expr::StaticGetVar(_, class_name, var_name) => {
                let id = env.static_envs[class_name];
                env.push_local_env(id);
                // Static methods are already validated and no checks are needed
                let returned = env.get(var_name).unwrap();
                env.pop_local_env();
                Ok(returned)
            }
        }
    }
}
