use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

#[derive(Debug, Clone)]
enum Stmt {
    VarDecl(String, Expr),
    Assign(String, Expr),
    If(Expr, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    EOI
}

#[derive(Debug, Clone)]
enum Expr {
    Ident(String),
    Data(DataType),
    BinOp(Box<Expr>, String, Box<Expr>),
}

#[derive(Debug, Clone)]
enum DataType {
    Number(f64),
    String(String),
}

fn build_ast(pair: pest::iterators::Pair<Rule>) -> Vec<Stmt> {
    assert_eq!(pair.as_rule(), Rule::program);

    println!("{:?}", pair);
    let x = pair.into_inner().map(build_stmt);

    x.collect()
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
        Rule::loop_stmt => {
            let mut inner = pair.into_inner();
            let cond = build_expr(inner.next().unwrap());
            let body: Vec<Stmt> = inner.map(build_stmt).collect();
            Stmt::While(cond, body)
        }
        Rule::EOI => {
            println!("EOI");
            Stmt::EOI
        }
        _ => {
            println!("SSS {:?}", pair);
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
                left = Expr::BinOp(Box::new(left), op.as_str().to_string(), Box::new(right));
            }
            left
        }
        Rule::term => build_expr(pair.into_inner().next().unwrap()),
        Rule::ident => Expr::Ident(pair.as_str().to_string()),
        Rule::number =>  {
            println!("called build_expr: {:?}", pair);
            Expr::Data(DataType::Number(pair.as_str().parse().unwrap()))
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
                    let r = match r {
                        DataType::Number(n) => n,
                        _ => unreachable!()
                    };

                    DataType::Number(match op.as_str() {
                        "+" => l + r,
                        "-" => l - r,
                        "*" => l * r,
                        "/" => l / r,
                        "%" => l % r,
                        ">" =>  {
                            match l > r {
                                true => 1.0,
                                false => 0.0,
                            }
                        },
                        "<" =>  {
                            match l < r {
                                true => 1.0,
                                false => 0.0,
                            }
                        },
                        _ => panic!("unknown operator"),
                    })
                },
                DataType::String(l) => {
                    panic!()
                }
            }
        }
    }
}

fn exec_stmt(stmt: &Stmt, env: &mut HashMap<String, DataType>) {
    match stmt {
        Stmt::VarDecl(name, expr) => {
            let val = eval_expr(expr, env);
            env.insert(name.clone(), val);
        }
        Stmt::Assign(name, expr) => {
            let val = eval_expr(expr, env);
            println!("{:?}", val);
            env.insert(name.clone(), val);
        }
        Stmt::If(cond, body) => {
            let res = eval_expr(cond, env);
            let res = match res {
                DataType::Number(n) => n,
                _ => unreachable!()
            };

            if res != 0.0 {
                for s in body {
                    exec_stmt(s, env);
                }
            }
        }
        Stmt::While(cond, body) => {

            fn is_true(cond: &Expr, env: &HashMap<String, DataType>) -> bool {
                let res = eval_expr(cond, env);
                let res = match res {
                    DataType::Number(n) => n,
                    _ => unreachable!()
                };

                res == 1.0
            }

            while is_true(&cond, &env) {

                for s in body {
                    exec_stmt(s, env);
                }
            }
        }
        Stmt::EOI => {}
    }
}

fn main() {
    let code = r#"
        X = 4.5;
        loop while X > -8
            if X > -8 then
                X = X - 1.5;
            end if
        end loop
    "#;

    let parsed = DSLParser::parse(Rule::program, code)
        .expect("parse failed")
        .next()
        .unwrap();

    let ast = build_ast(parsed);
    let mut env = HashMap::new();

    println!("{:#?}", ast);

    for stmt in &ast {
        exec_stmt(stmt, &mut env);
    }

    println!("Final env: {:?}", env);
}
