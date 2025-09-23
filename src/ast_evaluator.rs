use crate::ast::AST;
use crate::ast_nodes::{AssignOperator, Expr, MethodDef, Operator, Stmt, UnaryOp, Value};
use crate::env::Env;
use crate::utils::{num_op, str_op};
use std::io;
use std::io::Write;

impl AST {
    pub fn traverse(&self, env: &mut Env) {
        for stmt in &self.statements {
            self.exec_stmt(stmt, env);
        }
    }

    fn is_true(&self, cond: &Expr, env: &mut Env) -> bool {
        self.eval_expr(cond, env).as_num() != 0.0
    }

    fn eval_expr(&self, expr: &Expr, env: &mut Env) -> Value {
        match expr {
            Expr::Ident(name) => env.get(name).unwrap(),
            Expr::Data(n) => n.clone(),
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
                        Value::Number(r_num) => num_op(l_num, op, r_num),
                        Value::String(r_string) => Value::String(match op {
                            Operator::Add => l_num.to_string() + &*r_string,
                            _ => String::from("Nan"),
                        }),
                        Value::Bool(_r_bool) => {
                            Value::Bool(num_op(l_num, op, r.as_num()).as_bool())
                        }
                    },
                    Value::String(l_string) => match r {
                        Value::Number(r_num) => Value::String(match op {
                            Operator::Add => l_string + &r_num.to_string(),
                            _ => String::from("Nan"),
                        }),
                        Value::String(r_string) => str_op(l_string.as_str(), op, r_string.as_str()),
                        Value::Bool(_r_bool) => {
                            str_op(l_string.as_str(), op, r.to_string().as_str())
                        }
                    },
                    Value::Bool(l_bool) => match r {
                        Value::Number(r_num) => num_op(l.as_num(), op, r_num),
                        Value::String(r_string) => {
                            str_op(l.to_string().as_str(), op, r_string.as_str())
                        }
                        Value::Bool(r_bool) => Value::Bool(match op {
                            Operator::And => l_bool && r_bool,
                            Operator::Or => l_bool || r_bool,
                            _ => num_op(l.as_num(), op, r.as_num()).as_bool(),
                        }),
                    },
                }
            }
            Expr::Input(text) => {
                print!("{}", self.eval_expr(text, env));
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                match input.parse::<f64>() {
                    Ok(number) => Value::Number(number),
                    Err(_) => Value::String(input.to_string()),
                }
            }
            Expr::MethodCall(name, params) => {
                env.push_scope();

                let def = self.method_map.get(name).unwrap();
                self.define_method_params(def, params, env);

                let returned = match self.exec_body(&def.body, env) {
                    Some(returned_val) => returned_val,
                    None => panic!("No return for method call {}", name),
                };
                env.pop_scope();
                returned
            }
        }
    }

    fn exec_stmt(&self, stmt: &Stmt, env: &mut Env) -> Option<Value> {
        match stmt {
            Stmt::Assign(name, op, expr) => {
                let val = self.eval_expr(expr, env);

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
                None
            }
            Stmt::Increment(name) => {
                env.assign(name, env.get(name).unwrap() + Value::Number(1.0));
                None
            }
            Stmt::Decrement(name) => {
                env.assign(name, env.get(name).unwrap() - Value::Number(1.0));
                None
            }
            Stmt::If(cond, body) => {
                if self.is_true(cond, env) {
                    return self.exec_body(body, env);
                }
                None
            }
            Stmt::While(cond, body) => {
                while self.is_true(cond, env) {
                    if let Some(returned_val) = self.exec_body(body, env) {
                        return Some(returned_val)
                    }
                }
                None
            }
            Stmt::For(ident, start_num, end_num, body) => {
                let mut control = self.eval_expr(start_num, env).as_num();
                env.assign(ident, Value::Number(control));

                while control <= self.eval_expr(end_num, env).as_num() {
                    if let Some(returned_val) = self.exec_body(body, env) {
                        return Some(returned_val)
                    }

                    control = env.get(ident).unwrap().as_num();
                    control += 1.0;
                    env.assign(ident, Value::Number(control));
                }
                None
            }
            Stmt::Output(body) => {
                let mut output = String::new();

                for (i, expr) in body.iter().enumerate() {
                    if i > 0 {
                        output.push(' ');
                    }

                    match self.eval_expr(expr, env) {
                        Value::Number(n) => {
                            if n.abs() > 100000000000000000000.0 {
                                output.push_str(&format!("{:e}", n));
                            }
                            else {
                                output.push_str(&format!("{}", n));
                            }
                        },
                        Value::String(s) => output.push_str(s.trim()),
                        Value::Bool(b) => output.push_str(&b.to_string()),
                    }
                }

                println!("{}", &output);
                env.record_log(output);
                None
            }
            Stmt::Assert(expr, expected) => {
                assert_eq!(self.eval_expr(expr, env), self.eval_expr(expected, env));
                None
            }
            Stmt::MethodDeclaration(_name, _arg_names) => None,
            Stmt::MethodCall(name, params) => {
                env.push_scope();

                let def = self.method_map.get(name).unwrap();
                self.define_method_params(def, params, env);
                let returned_val = self.exec_body(&def.body, env);

                env.pop_scope();
                returned_val
            }
            Stmt::MethodReturn(expr) => Some(self.eval_expr(expr, env)),
            Stmt::EOI => None,
        }
    }

    fn define_method_params(&self, method_def: &MethodDef, params: &[Box<Expr>], env: &mut Env) {
        for (i, param) in params.iter().enumerate() {
            let value = self.eval_expr(param, env);
            env.define(method_def.args[i].clone(), value);
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
}
