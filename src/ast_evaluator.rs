use crate::ast::AST;
use crate::ast_nodes::{AssignOperator, AssignTarget, Expr, Function, Operator, Stmt, UnaryOp, Value};
use crate::common::{format_val, num_op, str_op};
use crate::env::{Env, EnvMode};
use std::cmp::max;
use std::collections::VecDeque;
use std::io;
use std::io::Write;

impl AST {
    pub fn traverse(&self, env: &mut Env) {
        for stmt in &self.statements {
            self.exec_stmt(stmt, env);
        }
    }

    fn exec_stmt(&self, stmt: &Stmt, env: &mut Env) -> Option<Value> {
        match stmt {
            Stmt::Assign(target, op, expr) => {
                let val = self.eval_expr(expr, env);
                self.exec_assign_stmt(target, op, val, env);
                None
            }
            Stmt::Increment(target) => {
                self.exec_assign_stmt(target, &AssignOperator::AssignAdd, Value::Number(1.0), env);
                None
            }
            Stmt::Decrement(target) => {
                self.exec_assign_stmt(target, &AssignOperator::AssignSubtract, Value::Number(1.0), env);
                None
            }
            Stmt::If { cond, then_branch, elifs, else_branch } => {
                if self.is_true(cond, env) {
                    return self.exec_body(then_branch, env);
                }

                for (elif_cond, elif_body) in elifs {
                    if self.is_true(elif_cond, env) {
                        return self.exec_body(elif_body, env);
                    }
                }

                if let Some(body) = else_branch {
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
                let mut control = self.eval_expr(start_num, env);
                env.assign(ident, control.clone());

                while control <= self.eval_expr(end_num, env) {
                    if let Some(returned_val) = self.exec_body(body, env) {
                        return Some(returned_val)
                    }

                    control = env.get(ident).unwrap();
                    control = control + Value::Number(1.0);
                    env.assign(ident, control.clone());
                }
                None
            }
            Stmt::Until(expr, body) => {
                while !self.is_true(expr, env) {
                    if let Some(returned_val) = self.exec_body(body, env) {
                        return Some(returned_val)
                    }
                }
                None
            }
            Stmt::Input(ident) => {
                let input = self.exec_input(ident, env);
                env.assign(ident.as_str(), input);
                None
            }
            Stmt::Output(body) => {
                let mut output = String::new();

                for (i, expr) in body.iter().enumerate() {
                    if i > 0 {
                        output.push(' ');
                    }

                    let val = self.eval_expr(expr, env);
                    format_val(&val, &mut output, env);
                }

                match &mut env.mode {
                    EnvMode::Release => println!("{}", &output),
                    EnvMode::Test { mock_inputs: _, logs } => Env::record_log(logs, output),
                }
                None
            }
            Stmt::Assert(expr, expected) => {
                assert_eq!(self.eval_expr(expr, env), self.eval_expr(expected, env));
                None
            }
            Stmt::MethodReturn(expr) => Some(self.eval_expr(expr, env)),
            Stmt::Expr(expr) => {
                self.eval_expr(expr, env);
                None
            }
            Stmt::FunctionDeclaration(_) => None,
            Stmt::ClassDeclaration(_) => None,
            Stmt::EOI => None,
        }
    }

    fn eval_expr(&self, expr: &Expr, env: &mut Env) -> Value {
        match expr {
            Expr::Ident(name) => env.get(name).unwrap(),
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

                let fn_def = match class_name.is_empty() {
                    false => self.class_map.get(class_name).unwrap().functions.get(name).unwrap(),
                    true => self.function_map.get(name).unwrap(),
                };

                self.exec_fn(fn_def, params, env).unwrap_or(Value::String(String::from("No return")))
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
            Expr::ClassNew(class_name, params) => {
                let class_def = self.class_map.get(class_name).unwrap();

                let id = env.create_local_env(class_name);
                env.push_local_env(id);

                for (i, param) in params.iter().enumerate() {
                    let val = self.eval_expr(param, env);
                    env.define(class_def.constructor.vars.get(i).unwrap().0.clone(), val);
                }

                env.pop_local_env();
                Value::Instance(class_name.clone(), id)
            }
            Expr::Call { expr, fn_name, params } => {
                if let Value::Instance(class_name, id) = self.eval_expr(expr, env) {
                    let fn_name = "this.".to_string() + fn_name;
                    let fn_def = self.class_map.get(&class_name).unwrap().functions.get(fn_name.as_str()).unwrap();

                    env.push_local_env(id);
                    let returned = self.exec_fn(fn_def, params, env);
                    env.pop_local_env();

                    return returned.unwrap_or(Value::Number(0.0));
                }
                panic!("Invalid call expression");
            }
        }
    }

    fn exec_assign_stmt(&self, target: &AssignTarget, op: &AssignOperator, val: Value, env: &mut Env) {
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
            }
            AssignTarget::Array(array_expr, index_expr) => {
                if let Value::Array(id) = self.eval_expr(array_expr, env) {
                    let index = self.eval_expr(index_expr, env).as_num() as i64;
                    let array = env.get_array_mut(&id);

                    if index < 0 {
                        panic!("Negative index");
                    }
                    let index = index as usize;
                    if index >= array.len() {
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
                } else {
                    panic!("Invalid index expression");
                }
            }
        }
    }

    fn exec_fn(&self, def: &Function, params: &[Expr], env: &mut Env) -> Option<Value> {
        env.push_scope();
        self.define_method_params(def, params, env);
        let returned = self.exec_body(&def.body, env);
        env.pop_scope();

        returned
    }

    fn is_true(&self, cond: &Expr, env: &mut Env) -> bool {
        self.eval_expr(cond, env).as_bool()
    }

    fn define_method_params(&self, method_def: &Function, params: &[Expr], env: &mut Env) {
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

    fn exec_input(&self, ask_string: &String, env: &mut Env) -> Value {
        let mut input;

        match &mut env.mode {
            EnvMode::Release => {
                print!("{}", ask_string);
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
}