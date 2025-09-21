use std::collections::HashMap;
use crate::ast::AST;
use crate::ast_nodes::{Value, Expr, MethodDef, Operator, Stmt};
use crate::Rule;
use crate::utils::fix_quotes_plain;

impl AST {
    pub fn build_ast(&mut self, pair: pest::iterators::Pair<Rule>) {
        self.method_map = HashMap::new();
        assert_eq!(pair.as_rule(), Rule::program);

        self.statements = pair.into_inner().map(|inner| self.build_stmt(inner)).collect();
    }

    fn build_stmt(&mut self, pair: pest::iterators::Pair<Rule>) -> Stmt {
        println!("{:?}", pair);
        match pair.as_rule() {
            Rule::assign => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str().to_string();
                let expr = self.build_expr(inner.next().unwrap());
                Stmt::Assign(ident, expr)
            }
            Rule::if_stmt => {
                let mut inner = pair.into_inner();
                let cond = self.build_expr(inner.next().unwrap());
                let body: Vec<Stmt> = inner.map(|inner| self.build_stmt(inner)).collect();
                Stmt::If(cond, body)
            }
            Rule::while_loop_stmt => {
                let mut inner = pair.into_inner();
                let cond = self.build_expr(inner.next().unwrap());
                let body: Vec<Stmt> = inner.map(|inner| self.build_stmt(inner)).collect();
                Stmt::While(cond, body)
            }
            Rule::for_loop_stmt => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str().to_string();
                let start_num = self.build_expr(inner.next().unwrap());
                let end_num = self.build_expr(inner.next().unwrap());
                let body: Vec<Stmt> = inner.map(|inner| self.build_stmt(inner)).collect();
                Stmt::For(ident, start_num, end_num, body)
            }
            Rule::output_stmt => {
                let inner = pair.into_inner();
                let body: Vec<Expr> = inner.map(|inner| self.build_expr(inner)).collect();
                Stmt::Output(body)
            }
            Rule::method_decl => {
                let mut inner = pair.into_inner();

                let method_name = inner.next().unwrap().as_str().to_string();
                let mut args = Vec::new();

                let try_inner = inner.clone().next().unwrap();
                match try_inner.as_rule() {
                    Rule::method_decl_param_list => {
                        inner.next(); // consume outer
                        let mut inner = try_inner.into_inner();

                        while let Some(arg) = inner.next() {
                            args.push(arg.as_str().to_string());
                        }
                    }
                    _ => {}
                }

                let body: Vec<Stmt> = inner.map(|inner| self.build_stmt(inner)).collect();

                self.method_map.insert(method_name.clone(), MethodDef {
                    args: args.clone(),
                    body,
                });

                Stmt::MethodDeclaration(method_name, args)
            },
            Rule::method_call => {
                let mut inner = pair.into_inner();

                let method_name = inner.next().unwrap().as_str().to_string();
                let args: Vec<Box<Expr>> = inner.map(|inner| Box::new(self.build_expr(inner))).collect();

                Stmt::MethodCall(method_name, args)
            },
            Rule::method_return => {
                let mut inner = pair.into_inner();
                Stmt::MethodReturn(self.build_expr(inner.next().unwrap()))
            },
            Rule::EOI => Stmt::EOI,
            _ => {
                println!("{:?}", pair);
                unreachable!()
            },
        }
    }

    fn build_expr(&mut self, pair: pest::iterators::Pair<Rule>) -> Expr {
        match pair.as_rule() {
            Rule::expr => {
                let mut inner = pair.into_inner();
                let mut left = self.build_expr(inner.next().unwrap());
                while let Some(op) = inner.next() {
                    let right = self.build_expr(inner.next().unwrap());

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
            Rule::term => self.build_expr(pair.into_inner().next().unwrap()),
            Rule::ident => Expr::Ident(pair.as_str().to_string()),
            Rule::number => Expr::Data(Value::Number(pair.as_str().parse().unwrap())),
            Rule::string => Expr::Data(Value::String(fix_quotes_plain(pair.as_str()))),
            Rule::method_call => {
                let mut inner = pair.into_inner();

                let method_name = inner.next().unwrap().as_str().to_string();
                let args: Vec<Box<Expr>> = inner.map(|inner| Box::new(self.build_expr(inner))).collect();

                Expr::MethodCall(method_name, args)
            }
            Rule::input => {
                let mut inner = pair.into_inner();
                let text = self.build_expr(inner.next().unwrap());
                Expr::Input(Box::new(text))
            },
            _ => unreachable!(),
        }
    }
}
