use crate::ast::AST;
use crate::ast_nodes::{AssignOperator, Expr, MethodDef, Operator, Stmt, Value};
use crate::utils::{to_num_bool, to_string_bool};
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::ops::{Add, Sub};



impl AST {
    pub fn traverse(&mut self) {
        for stmt in &self.statements.clone() {
            self.exec_stmt(stmt);
        }
    }

    fn eval_to_num(&mut self, cond: &Expr) -> f64 {
        self.eval_expr(cond).as_num()
    }

    fn is_true(&mut self, cond: &Expr) -> bool {
        self.eval_to_num(cond) != 0.0
    }

    fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Ident(name) => self.env.get(name).unwrap(),
            Expr::Data(n) => n.clone(),
            Expr::Input(text) => {
                match self.eval_expr(&text) {
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
                self.env.push_scope();

                let def = self.method_map.get(name).unwrap().clone();
                self.define_method_params(&def, params);

                let returned = match self.exec_body(&def.body) {
                    Some(returned_val) => returned_val,
                    None => panic!("No return for method call {}", name)
                };
                self.env.pop_scope();
                returned
            }
            Expr::BinOp(left, op, right) => {
                let l = self.eval_expr(left);
                let r = self.eval_expr(right);

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

    fn exec_stmt(&mut self, stmt: &Stmt) -> Option<Value> {
        match stmt {
            Stmt::Assign(name, op, expr) => {
                let val = self.eval_expr(expr);

                match op {
                    AssignOperator::Assign => self.env.assign(name, val),
                    AssignOperator::AssignAdd => self.env.assign(name, self.env.get(name).unwrap() + val),
                    AssignOperator::AssignSubtract => self.env.assign(name, self.env.get(name).unwrap() - val),
                }
                None
            }
            Stmt::If(cond, body) => {
                if self.is_true(cond) {
                    return self.exec_body(&body)
                }
                None
            }
            Stmt::While(cond, body) => {
                while self.is_true(cond) {
                    match self.exec_body(&body) {
                        Some(returned_val) => return Some(returned_val),
                        None => {}
                    }
                }
                None
            }
            Stmt::For(ident, start_num, end_num, body) => {
                let mut control = self.eval_to_num(&start_num);

                self.env.assign(ident, Value::Number(control));

                while control < self.eval_to_num(end_num) {
                    match self.exec_body(&body) {
                        Some(returned_val) => return Some(returned_val),
                        None => {}
                    }

                    control = self.env.get(ident).unwrap().as_num();
                    control += 1.0;
                    self.env.assign(ident, Value::Number(control));
                }
                None
            }
            Stmt::Output(body) => {
                let mut output = String::new();

                for (i, expr) in body.iter().enumerate() {
                    if i > 0 {
                        output.push(' ');
                    }

                    match self.eval_expr(&expr) {
                        Value::Number(n) => output.push_str(&n.to_string()),
                        Value::String(s) => output.push_str(&s),
                    }
                }
                println!("{}", output);
                None
            }
            Stmt::MethodDeclaration(_name, _arg_names) => None,
            Stmt::MethodCall(name, params) => {
                self.env.push_scope();

                let def = self.method_map.get(name).unwrap().clone();
                self.define_method_params(&def, params);
                let returned = self.exec_body(&def.body);

                self.env.pop_scope();
                returned
            }
            Stmt::MethodReturn(expr) => Some(self.eval_expr(expr)),
            Stmt::EOI => None,
        }
    }

    fn define_method_params(&mut self, method_def: &MethodDef, params: &Vec<Box<Expr>>) {
        for (i, param) in params.iter().enumerate() {
            let value = self.eval_expr(param);
            self.env.define(method_def.args[i].clone(), value);
        }
    }

    fn exec_body(&mut self, body: &Vec<Stmt>) -> Option<Value> {
        for s in body {
            match self.exec_stmt(s) {
                Some(returned_val) => return Some(returned_val),
                None => {}
            }
        }
        None
    }
}
