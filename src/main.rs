use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::utils::utils::*;

pub mod utils;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

#[derive(Debug, Clone)]
enum Stmt {
    Assign(String, Expr),
    If(Expr, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    For(String, Expr, Expr, Vec<Stmt>),
    Output(Vec<Expr>),
    EOI
}

#[derive(Debug, Clone)]
enum Expr {
    Ident(String),
    Data(DataType),
    BinOp(Box<Expr>, Operator, Box<Expr>), // Has to be boxed to avoid recursion in the enum definition
}

#[derive(Debug, Clone)]
enum DataType {
    Number(f64),
    String(String),
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Number(n) => write!(f, "Number({})", n),
            DataType::String(s) => write!(f, "String(\"{}\")", s),
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
}


fn build_ast(pair: pest::iterators::Pair<Rule>) -> Vec<Stmt> {
    assert_eq!(pair.as_rule(), Rule::program);
    pair.into_inner().map(build_stmt).collect()
}

fn build_stmt(pair: pest::iterators::Pair<Rule>) -> Stmt {
    println!("called build_stmt: {:?}", pair);
    match pair.as_rule() {
        Rule::assign => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = build_expr(inner.next().unwrap());
            Stmt::Assign(ident, expr)
        }
        Rule::if_stmt => {
            let mut inner = pair.into_inner();
            let cond = build_expr(inner.next().unwrap());
            let body: Vec<Stmt> = inner.map(build_stmt).collect();
            Stmt::If(cond, body)
        }
        Rule::while_loop_stmt => {
            let mut inner = pair.into_inner();
            let cond = build_expr(inner.next().unwrap());
            let body: Vec<Stmt> = inner.map(build_stmt).collect();
            Stmt::While(cond, body)
        }
        Rule::for_loop_stmt => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let start_num = build_expr(inner.next().unwrap());
            let end_num = build_expr(inner.next().unwrap());
            let body: Vec<Stmt> = inner.map(build_stmt).collect();
            Stmt::For(ident, start_num, end_num, body)
        }
        Rule::output_stmt => {
            let inner = pair.into_inner();
            let body: Vec<Expr> = inner.map(build_expr).collect();
            Stmt::Output(body)
        }
        Rule::EOI => Stmt::EOI,
        _ => {
            println!("Unexpected pair {:?}", pair);
            unreachable!()
        },
    }
}

fn build_expr(pair: pest::iterators::Pair<Rule>) -> Expr {
    println!("called build_expr: {:?}", pair);
    match pair.as_rule() {
        Rule::expr => {
            let mut inner = pair.into_inner();
            let mut left = build_expr(inner.next().unwrap());
            while let Some(op) = inner.next() {
                let right = build_expr(inner.next().unwrap());

                let op = match op.as_rule() {
                    Rule::add => Operator::Add,
                    Rule::subtract => Operator::Subtract,
                    Rule::multiply => Operator::Multiply,
                    Rule::divide => Operator::Divide,
                    Rule::power => Operator::Power,
                    Rule::modulo => Operator::Modulo,
                    Rule::greater => Operator::Greater,
                    Rule::less => Operator::Less,
                    Rule::greater_equal => Operator::GreaterEqual,
                    Rule::less_equal => Operator::LessEqual,
                    Rule::equal => Operator::Equal,
                    Rule::not_equal => Operator::NotEqual,
                    _ => unreachable!(),
                };

                left = Expr::BinOp(Box::new(left), op, Box::new(right));
            }
            left
        }
        Rule::term => build_expr(pair.into_inner().next().unwrap()),
        Rule::ident => Expr::Ident(pair.as_str().to_string()),
        Rule::number => Expr::Data(DataType::Number(pair.as_str().parse().unwrap())),
        Rule::string => {
            Expr::Data(DataType::String(fix_quotes_plain(pair.as_str())))
        },
        _ => unreachable!(),
    }
}

fn eval_expr(expr: &Expr, env: &HashMap<String, DataType>) -> DataType {
    match expr {
        Expr::Ident(name) => env.get(name).unwrap().clone(),
        Expr::Data(n) => n.clone(),
        Expr::BinOp(left, op, right) => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);

            match l {
                DataType::Number(l) => {
                    match r {
                        DataType::Number(r) => {
                            DataType::Number(match op {
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
                                Operator::NotEqual => to_num_bool(l != r)
                            })
                        },
                        DataType::String(_) => DataType::String(String::from("Nan")),
                    }
                },
                DataType::String(l) => {
                    match r {
                        DataType::Number(_) => DataType::String(String::from("Nan")),
                        DataType::String(r) => {
                            DataType::String(match op {
                                Operator::Add => l + &r,
                                Operator::Greater => to_string_bool(l > r),
                                Operator::Less => to_string_bool(l < r),
                                Operator::GreaterEqual => to_string_bool(l >= r),
                                Operator::LessEqual => to_string_bool(l <= r),
                                Operator::Equal => to_string_bool(l == r),
                                Operator::NotEqual => to_string_bool(l != r),
                                _ => String::from("Nan")
                            })
                        },
                    }
                },
            }
        }
    }
}

fn exec_stmt(stmt: &Stmt, env: &mut HashMap<String, DataType>) {
    match stmt {
        Stmt::Assign(name, expr) => {
            let val = eval_expr(expr, env);
            env.insert(name.clone(), val);
        }
        Stmt::If(cond, body) => {
            if is_true(cond, env) {
                for s in body {
                    exec_stmt(s, env);
                }
            }
        }
        Stmt::While(cond, body) => {
            while is_true(cond, env) {
                for s in body {
                    exec_stmt(s, env);
                }
            }
        }
        Stmt::For(ident, start_num, end_num, body) => {
            let mut control = eval_to_num(&start_num, env);

            env.insert(ident.clone(), DataType::Number(control));

            while control < eval_to_num(end_num, env) {
                for s in body {
                    exec_stmt(s, env);
                }

                control = to_num(env.get(ident).unwrap());
                control += 1.0;
                *env.get_mut(ident).unwrap() = DataType::Number(control);
            }
        }
        Stmt::Output(body) => {
            let mut output = String::new();

            for (i, expr) in body.iter().enumerate() {
                if i > 0 {
                    output.push(' ');
                }

                match eval_expr(&expr, env) {
                    DataType::Number(n) => output.push_str(&n.to_string()),
                    DataType::String(s) => output.push_str(&s),
                }
            }
            println!("{}", output);
        }
        Stmt::EOI => {},
    }
}

fn to_num(data: &DataType) -> f64 {
    match data {
        DataType::Number(n) => *n,
        _ => unreachable!()
    }
}

fn to_num_bool(data: bool) -> f64 {
    match data {
        true => 1.0,
        false => 0.0,
    }
}

fn to_string_bool(data: bool) -> String {
    match data {
        true => String::from("true"),
        false => String::from("false"),
    }
}

fn eval_to_num(cond: &Expr, env: &HashMap<String, DataType>) -> f64 {
    let res = eval_expr(cond, env);
    match res {
        DataType::Number(n) => n,
        _ => unreachable!()
    }
}

fn is_true(cond: &Expr, env: &HashMap<String, DataType>) -> bool {
    eval_to_num(cond, env) != 0.0
}

fn main() {
    let code = r#"
        X = 4.5
        A = "fsdfsdf"

        // comment
        loop I from -81 to 10
            output "sdads \"dsadsd\" " <= "4"
            if X >= -99 then
                X = X - 1.5
            end if
        end loop
    "#;

    let ast = compile(code);
    println!("{:#?}", ast);

    let env = run(ast);
    println!("Final env: {:?}", env);
}



fn compile(code: &str) -> Vec<Stmt> {
    let parsed = DSLParser::parse(Rule::program, code)
        .expect("parse failed")
        .next()
        .unwrap();

    build_ast(parsed)
}

fn run(ast: Vec<Stmt>) -> HashMap<String, DataType> {
    let mut env = HashMap::new();

    for stmt in &ast {
        exec_stmt(stmt, &mut env);
    }
    env
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let code = r#"
            X = 4.5
            A = "fsdfsdf"

            // comment
            loop I from -81 to 10
                output "sdads \"dsadsd\" " <= "4"
                if X >= -99 then
                    X = X - 1.5
                end if
            end loop
        "#;

        let env = compile_and_run(code);
        assert_env(&env, "A", &DataType::String("fsdfsdf".to_string()));
        assert_env(&env, "I", &DataType::Number(10.0));
        assert_env(&env, "X", &DataType::Number(-100.5));
    }

    fn compile_and_run(code: &str) -> HashMap<String, DataType> {
        let ast = compile(code);
        run(ast)
    }
}

fn assert_env(env: &HashMap<String, DataType>, var_name: &str, expected: &DataType) {
    assert!(env.contains_key(var_name), "Variable wasn't created");

    let var = env.get(var_name).unwrap();

    let correct = match var {
        DataType::Number(n) => {
            match expected {
                DataType::Number(e_n) => n == e_n,
                DataType::String(e_s) => panic!("Expected {} but got {}", e_s, n),
            }
        }
        DataType::String(s) => {
            match expected {
                DataType::Number(e_n) => panic!("Expected {} but got {}", e_n, s),
                DataType::String(e_s) => s == e_s
            }
        }
    };
    assert!(correct, "Environment variable wasn't as expected. Expected {} but got {}", expected, var);
}
