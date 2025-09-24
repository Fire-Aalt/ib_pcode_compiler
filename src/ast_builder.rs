use crate::ast::AST;
use crate::ast_nodes::{AssignOperator, Expr, MethodDef, Operator, Stmt, UnaryOp, Value};
use crate::common::fix_quotes_plain;
use crate::compiler::Rule;
use std::collections::HashMap;

impl AST {
    pub fn build_ast(&mut self, pair: pest::iterators::Pair<Rule>) {
        self.method_map = HashMap::new();
        assert_eq!(pair.as_rule(), Rule::program);

        self.statements = pair
            .into_inner()
            .map(|inner| self.build_stmt(inner))
            .collect();
    }

    fn build_stmt(&mut self, pair: pest::iterators::Pair<Rule>) -> Stmt {
        match pair.as_rule() {
            Rule::assign_stmt => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str().to_string();

                let mut index_expr = None;
                if inner.len() == 3 {
                    let idx_pair = inner.next().unwrap().into_inner().next().unwrap();
                    let idx_expr = build_expr(idx_pair);
                    index_expr = Some(idx_expr);
                }

                let op = match inner.next().unwrap().as_rule() {
                    Rule::assign => AssignOperator::Assign,
                    Rule::assign_add => AssignOperator::AssignAdd,
                    Rule::assign_subtract => AssignOperator::AssignSubtract,
                    Rule::assign_multiply => AssignOperator::AssignMultiply,
                    Rule::assign_divide => AssignOperator::AssignDivide,
                    _ => unreachable!(),
                };
                let expr = build_expr(inner.next().unwrap());
                Stmt::Assign(ident, index_expr, op, expr)
            }
            Rule::increment_stmt => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str().to_string();
                Stmt::Increment(ident)
            }
            Rule::decrement_stmt => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str().to_string();
                Stmt::Decrement(ident)
            }
            Rule::if_stmt => {
                let mut inner = pair.into_inner();
                let cond = build_expr(inner.next().unwrap());

                let mut then_branch: Vec<Stmt> = Vec::new();
                let mut elifs: Vec<(Expr, Vec<Stmt>)> = Vec::new();
                let mut else_branch: Option<Vec<Stmt>> = None;

                for p in inner {
                    match p.as_rule() {
                        Rule::elif_clause => {
                            let mut elif_inner = p.into_inner();
                            let elif_cond = build_expr(elif_inner.next().unwrap());

                            let mut elif_body = Vec::new();
                            for sp in elif_inner {
                                elif_body.push(self.build_stmt(sp));
                            }
                            elifs.push((elif_cond, elif_body));
                        }
                        Rule::else_clause => {
                            let mut else_body = Vec::new();
                            for sp in p.into_inner() {
                                else_body.push(self.build_stmt(sp));
                            }
                            else_branch = Some(else_body);
                        }
                        _ => then_branch.push(self.build_stmt(p)),
                    }
                }
                Stmt::If {
                    cond,
                    then_branch,
                    elifs,
                    else_branch,
                }
            }
            Rule::while_loop_stmt => {
                let mut inner = pair.into_inner();
                let cond = build_expr(inner.next().unwrap());
                let body: Vec<Stmt> = inner.map(|inner| self.build_stmt(inner)).collect();
                Stmt::While(cond, body)
            }
            Rule::for_loop_stmt => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str().to_string();
                let start_num = build_expr(inner.next().unwrap());
                let end_num = build_expr(inner.next().unwrap());
                let body: Vec<Stmt> = inner.map(|inner| self.build_stmt(inner)).collect();
                Stmt::For(ident, start_num, end_num, body)
            }
            Rule::loop_until_stmt => {
                let mut inner = pair.into_inner();
                let expr = build_expr(inner.next().unwrap());
                let body: Vec<Stmt> = inner.map(|inner| self.build_stmt(inner)).collect();
                Stmt::Until(expr, body)
            }
            Rule::input_stmt => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str().to_string();
                Stmt::Input(ident)
            }
            Rule::output_stmt => {
                let inner = pair.into_inner();
                let body: Vec<Expr> = inner.map(|inner| build_expr(inner)).collect();
                Stmt::Output(body)
            }
            Rule::assert_stmt => {
                let mut inner = pair.into_inner();
                let expr = build_expr(inner.next().unwrap());
                let expected = build_expr(inner.next().unwrap());
                Stmt::Assert(expr, expected)
            }
            Rule::method_decl => {
                let mut inner = pair.into_inner();

                let method_name = inner.next().unwrap().as_str().to_string();
                let mut args = Vec::new();

                let try_inner = inner.clone().next().unwrap();
                if try_inner.as_rule() == Rule::method_decl_param_list {
                    inner.next(); // consume outer
                    let inner = try_inner.into_inner();

                    for arg in inner {
                        args.push(arg.as_str().to_string());
                    }
                }

                let body: Vec<Stmt> = inner.map(|inner| self.build_stmt(inner)).collect();

                self.method_map.insert(
                    method_name.clone(),
                    MethodDef {
                        args: args.clone(),
                        body,
                    },
                );

                Stmt::MethodDeclaration(method_name, args)
            }
            Rule::method_call => {
                let mut inner = pair.into_inner();

                let method_name = inner.next().unwrap().as_str().to_string();
                let args: Vec<Box<Expr>> = inner
                    .map(|inner| Box::new(build_expr(inner)))
                    .collect();

                Stmt::MethodCall(method_name, args)
            }
            Rule::method_return => {
                let mut inner = pair.into_inner();
                Stmt::MethodReturn(build_expr(inner.next().unwrap()))
            }
            Rule::EOI => Stmt::EOI,
            _ => unreachable!(),
        }
    }
}

fn build_expr(pair: pest::iterators::Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::expr
        | Rule::logical_or
        | Rule::logical_and
        | Rule::comparison
        | Rule::add_sub
        | Rule::mul_div => {
            let mut inner = pair.into_inner();
            let mut left = build_expr(inner.next().unwrap());
            while let Some(op_pair) = inner.next() {
                let right = build_expr(inner.next().unwrap());
                let op = match op_pair.as_rule() {
                    Rule::add => Operator::Add,
                    Rule::subtract => Operator::Subtract,
                    Rule::multiply => Operator::Multiply,
                    Rule::divide => Operator::Divide,
                    Rule::int_divide => Operator::IntDivide,
                    Rule::power => Operator::Power,
                    Rule::modulo => Operator::Modulo,
                    Rule::greater => Operator::Greater,
                    Rule::less => Operator::Less,
                    Rule::greater_equal => Operator::GreaterEqual,
                    Rule::less_equal => Operator::LessEqual,
                    Rule::equal => Operator::Equal,
                    Rule::not_equal => Operator::NotEqual,
                    Rule::and => Operator::And,
                    Rule::or => Operator::Or,
                    _ => unreachable!(),
                };
                left = Expr::BinOp(Box::new(left), op, Box::new(right));
            }
            left
        }
        Rule::pow => {
            let mut inner = pair.into_inner();
            let left = build_expr(inner.next().unwrap());
            if let Some(_op_pair) = inner.next() {
                let right = build_expr(inner.next().unwrap());
                Expr::BinOp(Box::new(left), Operator::Power, Box::new(right))
            } else {
                left
            }
        }
        Rule::unary => {
            let mut parts: Vec<_> = pair.into_inner().collect();
            let mut expr = build_expr(parts.pop().unwrap());

            while let Some(op_pair) = parts.pop() {
                let op = match op_pair.as_rule() {
                    Rule::subtract => UnaryOp::Neg,
                    Rule::not => UnaryOp::Not,
                    _ => unreachable!(),
                };
                expr = Expr::Unary(op, Box::new(expr));
            }
            expr
        }
        Rule::term => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap().into_inner().next().unwrap();

            let mut node = match first.as_rule() {
                Rule::ident => Expr::Ident(first.as_str().to_string()),
                Rule::number => Expr::Data(Value::Number(first.as_str().parse().unwrap())),
                Rule::string => Expr::Data(Value::String(fix_quotes_plain(first.as_str()))),
                Rule::bool => Expr::Data(Value::Bool(first.as_str().parse().unwrap())),
                Rule::array => {
                    let inner = first.into_inner();
                    let data: Vec<Expr> = inner
                        .map(|inner| build_expr(inner))
                        .collect();
                    Expr::Array(data)
                }
                Rule::method_call => {
                    let mut inner = first.into_inner();

                    let method_name = inner.next().unwrap().as_str().to_string();
                    let args: Vec<Box<Expr>> = inner
                        .map(|inner| Box::new(build_expr(inner)))
                        .collect();
                    Expr::MethodCall(method_name, args)
                }
                Rule::input_call => {
                    let mut inner = first.into_inner();
                    let text = build_expr(inner.next().unwrap());
                    Expr::Input(Box::new(text))
                }
                Rule::div_call => {
                    let mut inner = first.into_inner();
                    let left = build_expr(inner.next().unwrap());
                    let right = build_expr(inner.next().unwrap());
                    Expr::Div(Box::new(left), Box::new(right))
                }
                _ => build_expr(first),
            };

            for post in inner {
                let post = post.into_inner().next().unwrap();

                match post.as_rule() {
                    Rule::substring_call => {
                        let mut inner = post.into_inner();
                        let start = build_expr(inner.next().unwrap());
                        let end = build_expr(inner.next().unwrap());
                        node = Expr::SubstringCall { expr: Box::new(node), start: Box::new(start), end: Box::new(end) };
                    }
                    Rule::call => {
                        let args: Vec<Expr> = post.into_inner()
                            .map(|p| build_expr(p))
                            .collect();
                        node = Expr::Call(Box::new(node), args);
                    }
                    Rule::index => {
                        let idx_pair = post.into_inner().next().unwrap();
                        let idx_expr = build_expr(idx_pair);
                        node = Expr::Index(Box::new(node), Box::new(idx_expr));
                    }
                    _ => unreachable!(),
                }
            }
            node
        }
        _ => unreachable!(),
    }
}
