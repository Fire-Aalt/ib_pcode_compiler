use crate::ast::{hash, AST};
use crate::common::fix_quotes_plain;
use crate::compiler::Rule;
use crate::data::Value;
use crate::data::ast_nodes::{Expr, ExprNode, Operand, UnaryOp};
use crate::data::diagnostic::LineInfo;
use pest::iterators::Pair;

fn expr_node(line: LineInfo, expr: Expr) -> ExprNode {
    ExprNode {
        line_info: line,
        expr,
    }
}

impl AST {
    pub fn build_expr(&mut self, pair: Pair<Rule>) -> ExprNode {
        match pair.as_rule() {
            Rule::expr
            | Rule::logical_or
            | Rule::logical_and
            | Rule::comparison
            | Rule::add_sub
            | Rule::mul_div => {
                let mut inner = pair.into_inner();
                let mut left = self.build_expr(inner.next().unwrap());

                while let Some(op_pair) = inner.next() {
                    let line = self.as_line_info(&op_pair);

                    let right = self.build_expr(inner.next().unwrap());
                    let op = match op_pair.as_rule() {
                        Rule::add => Operand::Add,
                        Rule::subtract => Operand::Subtract,
                        Rule::multiply => Operand::Multiply,
                        Rule::divide => Operand::Divide,
                        Rule::int_divide => Operand::IntDivide,
                        Rule::power => Operand::Power,
                        Rule::modulo => Operand::Modulo,
                        Rule::greater => Operand::Greater,
                        Rule::less => Operand::Less,
                        Rule::greater_equal => Operand::GreaterEqual,
                        Rule::less_equal => Operand::LessEqual,
                        Rule::equal => Operand::Equal,
                        Rule::not_equal => Operand::NotEqual,
                        Rule::and => Operand::And,
                        Rule::or => Operand::Or,
                        _ => unreachable!(),
                    };
                    left = expr_node(line, Expr::BinOp(Box::new(left), op, Box::new(right)));
                }
                left
            }
            Rule::pow => {
                let mut inner = pair.into_inner();
                let left = self.build_expr(inner.next().unwrap());
                if let Some(op_pair) = inner.next() {
                    let line = self.as_line_info(&op_pair);
                    let right = self.build_expr(inner.next().unwrap());

                    expr_node(
                        line,
                        Expr::BinOp(Box::new(left), Operand::Power, Box::new(right)),
                    )
                } else {
                    left
                }
            }
            Rule::unary => {
                let mut parts: Vec<_> = pair.into_inner().collect();
                let mut expr = self.build_expr(parts.pop().unwrap());

                while let Some(op_pair) = parts.pop() {
                    let line = self.as_line_info(&op_pair);

                    let op = match op_pair.as_rule() {
                        Rule::subtract => UnaryOp::Neg,
                        Rule::not => UnaryOp::Not,
                        _ => unreachable!(),
                    };
                    expr = expr_node(line, Expr::Unary(op, Box::new(expr)));
                }
                expr
            }
            Rule::term => self.build_term(pair),
            _ => unreachable!(),
        }
    }

    pub fn build_term(&mut self, pair: Pair<Rule>) -> ExprNode {
        let mut inner = pair.into_inner();
        let first = inner.next().unwrap().into_inner().next().unwrap();
        let line = self.as_line_info(&first);

        let mut node = match first.as_rule() {
            Rule::ident => expr_node(line, Expr::Ident(self.hash(first.as_str()))),
            Rule::number => expr_node(
                line,
                Expr::Data(Value::Number(first.as_str().parse().unwrap())),
            ),
            Rule::string => expr_node(
                line,
                Expr::Data(Value::String(fix_quotes_plain(first.as_str()))),
            ),
            Rule::bool => expr_node(
                line,
                Expr::Data(Value::Bool(first.as_str().parse().unwrap())),
            ),
            Rule::undefined => expr_node(line, Expr::Data(Value::Undefined)),
            Rule::array => {
                let inner = first.into_inner();
                let data = inner.map(|inner| self.build_expr(inner)).collect();
                expr_node(line, Expr::Array(data))
            }
            Rule::method_call => {
                let mut inner = first.into_inner();

                let method_name = inner.next().unwrap().as_str();
                let mut params: Vec<ExprNode> = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|inner| self.build_expr(inner))
                    .collect();

                if method_name == "div" {
                    assert_eq!(params.len(), 2);
                    params.reverse();
                    let left = params.pop().unwrap();
                    let right = params.pop().unwrap();
                    return expr_node(line, Expr::Div(Box::new(left), Box::new(right)));
                }
                if method_name == "input" {
                    assert!(params.len() <= 1);

                    let mut text = expr_node(line.clone(), Expr::Data(Value::String("".to_owned())));
                    if params.len() == 1 {
                        text = params.pop().unwrap();
                    }
                    return expr_node(line, Expr::Input(Box::new(text)));
                }

                expr_node(line, Expr::LocalMethodCall(self.hash(method_name), params))
            }
            Rule::class_new => {
                let mut inner = first.into_inner();

                let name = inner.next().unwrap().as_str();
                let args = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|inner| self.build_expr(inner))
                    .collect();
                expr_node(line, Expr::ClassNew(self.hash(name), args))
            }
            Rule::class_ident => expr_node(line, Expr::Ident(self.hash(first.as_str()))),
            _ => self.build_expr(first),
        };

        for post in inner {
            let post = post.into_inner().next().unwrap();
            let line = self.as_line_info(&post);

            match post.as_rule() {
                Rule::class_var => {
                    let mut inner = post.into_inner();
                    let mut var_name = self.hash(inner.next().unwrap().as_str());
                    var_name.this_keyword = true;

                    if let Expr::Ident(static_class_name) = &node.expr {
                        if self.static_classes.contains(static_class_name) {
                            node = expr_node(line.clone(), Expr::StaticGetVar(static_class_name.clone(), var_name));
                            continue // static early out
                        } else {
                            node = expr_node(line.clone(), Expr::Ident(static_class_name.clone()));
                        }
                    };

                    if !matches!(node.expr, Expr::StaticGetVar(_, _)) {
                        if var_name == hash("this.length") {
                            node = expr_node(line, Expr::LengthCall(Box::new(node)));
                            continue
                        } else {
                            node = expr_node(line, Expr::ClassGetVar(Box::new(node), var_name));
                        }
                    }
                }
                Rule::class_call => {
                    let mut inner = post.into_inner();

                    let fn_name = inner.next().unwrap().as_str();
                    let mut fn_name_hash = self.hash(fn_name);
                    fn_name_hash.this_keyword = true;

                    let mut params: Vec<ExprNode> = inner
                        .next()
                        .unwrap()
                        .into_inner()
                        .map(|p| self.build_expr(p))
                        .collect();

                    if let Expr::Ident(static_class_name) = &node.expr {
                        if self.static_classes.contains(static_class_name) {
                            if static_class_name == hash("Math") && fn_name == "random" {
                                assert_eq!(params.len(), 0);
                                node = expr_node(line, Expr::MathRandom);
                            } else {
                                node = expr_node(line, Expr::StaticMethodCall(static_class_name.clone(), fn_name_hash, params));
                            }
                            continue // static early out
                        } else {
                            node = expr_node(line.clone(), Expr::Ident(static_class_name.clone()));
                        }
                    };

                    if !matches!(node.expr, Expr::StaticMethodCall(_, _, _)) {
                        if fn_name == "substring" {
                            assert_eq!(params.len(), 2);
                            params.reverse();
                            let start = params.pop().unwrap();
                            let end = params.pop().unwrap();

                            node = expr_node(
                                line.clone(),
                                Expr::SubstringCall {
                                    expr: Box::new(node),
                                    start: Box::new(start),
                                    end: Box::new(end),
                                },
                            );
                            continue
                        } else {
                            node = expr_node(
                                line,
                                Expr::ClassMethodCall {
                                    expr: Box::new(node),
                                    fn_name: fn_name_hash,
                                    params,
                                },
                            );
                        }
                    }
                }
                Rule::index => {
                    let idx_pair = post.into_inner().next().unwrap();
                    let idx_expr = self.build_expr(idx_pair);
                    node = expr_node(line, Expr::Index(Box::new(node), Box::new(idx_expr)));
                }
                _ => unreachable!(),
            }
        }
        node
    }
}
