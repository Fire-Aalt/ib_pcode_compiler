use std::collections::VecDeque;
use crate::ast::AST;
use crate::common::{num_op, str_op};
use crate::data::ast_nodes::{Expr, ExprNode, Line, Operator, UnaryOp};
use crate::data::Value;
use crate::env::Env;

#[derive(Debug)]
pub struct Diagnostic<'a> {
    pub line: &'a Line,
    pub message: String,
}

impl AST {
    pub fn eval_expr(&self, expr_node: &ExprNode, env: &mut Env) -> Value {
        match &expr_node.expr {
            Expr::Ident(name) => {
                //println!("{}", name);
                //println!("{:?}", env);
                env.get(name).unwrap()
            },
            Expr::Data(n) => n.clone(),
            Expr::Array(data) => {
                let mut array = VecDeque::new();

                for expr in data {
                    array.push_back(self.eval_expr(expr, env))
                }

                let id = env.create_array(array);
                Value::Array(id)
            }
            Expr::Unary(op, expr) => {
                let value = self.eval_expr(expr, env);
                match op {
                    UnaryOp::Neg => -value,
                    UnaryOp::Not => !value,
                }
            }
            Expr::BinOp(left, op, right) => {
                let l = self.eval_expr(left, env);
                let r = self.eval_expr(right, env);

                match l {
                    Value::Number(l_num) => match r {
                        Value::Number(_) => num_op(l, op, r),
                        Value::Bool(_) => num_op(l, op, r),
                        Value::String(r_string) => Value::String(match op {
                            Operator::Add => l_num.to_string() + &*r_string,
                            _ => String::from("Nan"),
                        }),
                        _ => Value::String(String::from("Nan"))
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
                        _ => Value::String(String::from("Nan"))
                    },
                    Value::String(l_string) => match r {
                        Value::Number(r_num) => Value::String(match op {
                            Operator::Add => l_string + &r_num.to_string(),
                            _ => String::from("Nan"),
                        }),
                        Value::Bool(_) => str_op(l_string.as_str(), op, r.to_string().as_str()),
                        Value::String(r_string) => str_op(l_string.as_str(), op, r_string.as_str()),
                        _ => Value::String(String::from("Nan"))
                    },
                    _ => Value::String(String::from("Nan"))
                }
            }
            Expr::Input(text) => self.exec_input(&self.eval_expr(text, env).to_string(), env),
            Expr::Div(left, right) => {
                let left = self.eval_expr(left, env).as_num();
                let right = self.eval_expr(right, env).as_num();

                Value::Number((left as i64 / right as i64) as f64)
            }
            Expr::MethodCall(name, params) => {
                let class_name = &env.get_local_env().class_name;

                let fn_def = self.get_function(class_name, name);

                let params = params.iter().map(|p| self.eval_expr(p, env)).collect::<Vec<_>>();
                self.exec_fn(fn_def, &params, env).unwrap_or(Value::String(String::from("No return")))
            }
            Expr::SubstringCall { expr, start, end } => {
                if let Value::String(s) = self.eval_expr(expr, env) {
                    let start = self.eval_expr(start, env).as_num() as usize;
                    let end = self.eval_expr(end, env).as_num() as usize;

                    Value::String(s[start..end].to_string())
                } else {
                    panic!("Substring call expression is not string");
                }
            }
            Expr::Index(left, index) => {
                if let Value::Array(id) = self.eval_expr(left, env) {
                    let index = self.eval_expr(index, env).as_num() as i64;
                    let array = env.get_array_mut(&id);

                    if index < 0 || index >= array.len() as i64 {
                        return Value::String("undefined".to_string())
                    }

                    return array[index as usize].clone()
                }
                panic!("Invalid index expression");
            }
            Expr::ClassNew(class_name_hash, params) => {
                let class_def = self.get_class(class_name_hash);
                let id = env.create_local_env(class_name_hash.clone());

                env.push_local_env(id);
                // Define temp arg values
                for (i, param) in params.iter().enumerate() {
                    let arg_name_hash = &class_def.constructor.args[i];
                    let val = self.eval_expr(param, env);

                    env.define(arg_name_hash, val);
                }

                // Constructor
                for (name_hash, expr) in &class_def.constructor.constructors {
                    let val = self.eval_expr(expr, env);
                    env.define(name_hash, val);
                }

                // Undefine temp arg values
                for arg_name_hash in &class_def.constructor.args {
                    env.undefine(arg_name_hash);
                }
                env.pop_local_env();

                Value::Instance(id)
            }
            Expr::Call { expr, fn_name, params } => {
                if let Value::Instance(id) = self.eval_expr(expr, env) {
                    let class_name_hash = env.get_class_name_hash(&id);
                    let fn_def = self.get_function(class_name_hash, fn_name);

                    let params = params.iter().map(|p| self.eval_expr(p, env)).collect::<Vec<_>>();

                    env.push_local_env(id);
                    let returned = self.exec_fn(fn_def, &params, env);
                    env.pop_local_env();

                    return returned.unwrap_or(Value::Number(0.0));
                }
                panic!("Invalid call expression");
            }
        }
    }
}