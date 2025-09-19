use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

#[derive(Debug, Clone)]
enum Stmt {
    Assign(String, Expr),
    If(Expr, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    For(String, Expr, Expr, Vec<Stmt>),
    EOI
}

#[derive(Debug, Clone)]
enum Expr {
    Ident(String),
    Data(DataType),
    BinOp(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug, Clone)]
enum DataType {
    Number(f64),
    String(String),
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
                DataType::String(l) => {
                    panic!()
                }
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
            if eval_to_num(cond, &env) != 0.0 {
                for s in body {
                    exec_stmt(s, env);
                }
            }
        }
        Stmt::While(cond, body) => {
            while is_true(&cond, &env) {
                for s in body {
                    exec_stmt(s, env);
                }
            }
        }
        Stmt::For(ident, start_num, end_num, body) => {
            let mut control = eval_to_num(&start_num, &env);

            env.insert(ident.clone(), DataType::Number(control));

            while control < eval_to_num(end_num, &env) {
                for s in body {
                    exec_stmt(s, env);
                }

                control = to_num(env.get(ident).unwrap());
                control += 1.0;
                *env.get_mut(ident).unwrap() = DataType::Number(control);
            }
        }
        Stmt::EOI => {}
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

fn eval_to_num(cond: &Expr, env: &HashMap<String, DataType>) -> f64 {
    let res = eval_expr(cond, env);
    match res {
        DataType::Number(n) => n,
        _ => unreachable!()
    }
}

fn is_true(cond: &Expr, env: &HashMap<String, DataType>) -> bool {
    eval_to_num(cond, env) == 1.0
}

fn main() {
    let code = r#"
        X = 4.5;
        loop while X > -8
            if X == -8 then
                X = X - 1.5;
            end if
        end loop
    "#;

    let code = r#"
        X = 4.5;
        loop I from -81 to 10
            if X >= -99 then
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
