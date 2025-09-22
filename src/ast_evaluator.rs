use crate::ast::AST;
use crate::ast_nodes::{AssignOperator, Expr, MethodDef, Operator, Stmt, Value};
use crate::utils::{to_num_bool, to_string_bool};
use std::io;
use std::io::Write;
use crate::env::Env;

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
            Expr::Input(text) => {
                match self.eval_expr(text, env) {
                    Value::Number(n) => print!("{}", n),
                    Value::String(s) => print!("{}", s),
                }
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
                    None => panic!("No return for method call {}", name)
                };
                env.pop_scope();
                returned
            }
            Expr::BinOp(left, op, right) => {
                let l = self.eval_expr(left, env);
                let r = self.eval_expr(right, env);

                match l {
                    Value::Number(l) => match r {
                        Value::Number(r) => Value::Number(match op {
                            Operator::Add => l + r,
                            Operator::Subtract => l - r,
                            Operator::Multiply => l * r,
                            Operator::Divide => l / r,
                            Operator::Power => l.powf(r),
                            Operator::Modulo => l % r,
                            Operator::Greater => to_num_bool(l > r),
                            Operator::Less => to_num_bool(l < r),
                            Operator::GreaterEqual => to_num_bool(l >= r),
                            Operator::LessEqual => to_num_bool(l <= r),
                            Operator::Equal => to_num_bool(l == r),
                            Operator::NotEqual => to_num_bool(l != r),
                        }),
                        Value::String(r) => Value::String(match op {
                            Operator::Add => l.to_string() + &*r,
                            _ => String::from("Nan"),
                        }),
                    },
                    Value::String(l) => match r {
                        Value::Number(r) => Value::String(match op {
                            Operator::Add => l + &*r.to_string(),
                            _ => String::from("Nan"),
                        }),
                        Value::String(r) => Value::String(match op {
                            Operator::Add => l + &r,
                            Operator::Greater => to_string_bool(l > r),
                            Operator::Less => to_string_bool(l < r),
                            Operator::GreaterEqual => to_string_bool(l >= r),
                            Operator::LessEqual => to_string_bool(l <= r),
                            Operator::Equal => to_string_bool(l == r),
                            Operator::NotEqual => to_string_bool(l != r),
                            _ => String::from("Nan"),
                        }),
                    },
                }
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
                    AssignOperator::AssignSubtract => env.assign(name, env.get(name).unwrap() - val),
                    AssignOperator::AssignMultiply => env.assign(name, env.get(name).unwrap() * val),
                    AssignOperator::AssignDivide => env.assign(name, env.get(name).unwrap() / val)
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
                    return self.exec_body(body, env)
                }
                None
            }
            Stmt::While(cond, body) => {
                while self.is_true(cond, env) {
                    match self.exec_body(body, env) {
                        Some(returned_val) => return Some(returned_val),
                        None => {}
                    }
                }
                None
            }
            Stmt::For(ident, start_num, end_num, body) => {
                let mut control = self.eval_expr(start_num, env).as_num();
                env.assign(ident, Value::Number(control));

                while control < self.eval_expr(end_num, env).as_num() {
                    match self.exec_body(body, env) {
                        Some(returned_val) => return Some(returned_val),
                        None => {}
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
                        Value::Number(n) => output.push_str(&n.to_string()),
                        Value::String(s) => output.push_str(&s.trim()),
                    }
                }
                println!("{}", output);
                None
            }
            Stmt::MethodDeclaration(_name, _arg_names) => None,
            Stmt::MethodCall(name, params) => {
                env.push_scope();

                let def = self.method_map.get(name).unwrap();
                self.define_method_params(&def, params, env);
                let returned_val = self.exec_body(&def.body, env);

                env.pop_scope();
                returned_val
            }
            Stmt::MethodReturn(expr) => Some(self.eval_expr(expr, env)),
            Stmt::EOI => None,
        }
    }

    fn define_method_params(&self, method_def: &MethodDef, params: &Vec<Box<Expr>>, env: &mut Env) {
        for (i, param) in params.iter().enumerate() {
            let value = self.eval_expr(param, env);
            env.define(method_def.args[i].clone(), value);
        }
    }

    fn exec_body(&self, body: &Vec<Stmt>, env: &mut Env) -> Option<Value> {
        for stmt in body {
            match self.exec_stmt(stmt, env) {
                Some(returned_val) => return Some(returned_val),
                None => {}
            }
        }
        None
    }
}
