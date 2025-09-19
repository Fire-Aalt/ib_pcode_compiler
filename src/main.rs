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
    Number(f64),
    BinOp(Box<Expr>, String, Box<Expr>),
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
        Rule::var_decl => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = build_expr(inner.next().unwrap());
            Stmt::VarDecl(ident, expr)
        }
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
            Expr::Number(pair.as_str().parse().unwrap())
        },
        _ => unreachable!(),
    }
}

fn eval_expr(expr: &Expr, env: &mut HashMap<String, f64>) -> f64 {
    match expr {
        Expr::Ident(name) => *env.get(name).unwrap_or(&0.0),
        Expr::Number(n) => *n,
        Expr::BinOp(left, op, right) => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            match op.as_str() {
                "+" => l + r,
                "-" => l - r,
                "*" => l * r,
                "/" => l / r,
                _ => panic!("unknown operator"),
            }
        }
    }
}

fn exec_stmt(stmt: &Stmt, env: &mut HashMap<String, f64>) {
    match stmt {
        Stmt::VarDecl(name, expr) => {
            let val = eval_expr(expr, env);
            env.insert(name.clone(), val);
        }
        Stmt::Assign(name, expr) => {
            let val = eval_expr(expr, env);
            env.insert(name.clone(), val);
        }
        Stmt::If(cond, body) => {
            if eval_expr(cond, env) != 0.0 {
                for s in body {
                    exec_stmt(s, env);
                }
            }
        }
        Stmt::While(cond, body) => {
            while eval_expr(cond, env) != 0.0 {
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
        let x = 4.5;
        while x {
            if x {
                x = x - 1.5;
            }
        }
    "#;

    let parsed = DSLParser::parse(Rule::program, code)
        .expect("parse failed")
        .next()
        .unwrap();

    let ast = build_ast(parsed);
    let mut env = HashMap::new();

    for stmt in &ast {
        exec_stmt(stmt, &mut env);
    }

    println!("Final env: {:?}", env);
}
