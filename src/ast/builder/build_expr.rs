use crate::ast::{hash_const, AST};
use crate::common::fix_quotes_plain;
use crate::compiler::Rule;
use crate::data::ast_nodes::{Expr, ExprNode, NativeMethod, Operand, UnaryOp};
use crate::data::diagnostic::LineInfo;
use crate::data::{NameHash, Value};
use pest::iterators::Pair;

fn expr_node(line: &LineInfo, expr: Expr) -> ExprNode {
    ExprNode {
        line_info: line.clone(),
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
                    let line = &self.as_line_info(&op_pair);

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
                    let line = &self.as_line_info(&op_pair);
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
                    let line = &self.as_line_info(&op_pair);

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
        let line = &self.as_line_info(&first);

        let mut node = match first.as_rule() {
            Rule::ident => Expr::Ident(self.hash(first.as_str())),
            Rule::number => Expr::Data(Value::Number(first.as_str().parse().unwrap())),
            Rule::string => Expr::Data(Value::String(fix_quotes_plain(first.as_str()))),
            Rule::bool => Expr::Data(Value::Bool(first.as_str().parse().unwrap())),
            Rule::undefined => Expr::Data(Value::Undefined),
            Rule::array => {
                let inner = first.into_inner();
                let data = inner.map(|inner| self.build_expr(inner)).collect();
                Expr::Array(data)
            }
            Rule::method_call => {
                let mut inner = first.into_inner();

                let method_name = self.hash(inner.next().unwrap().as_str());
                let params: Vec<ExprNode> = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|inner| self.build_expr(inner))
                    .collect();

                const DIV: NameHash = hash_const("div");
                const INPUT: NameHash = hash_const("input");

                match method_name {
                    DIV => Expr::NativeMethodCall(NativeMethod::Div, None, params),
                    INPUT => Expr::NativeMethodCall(NativeMethod::Input, None, params),
                    _ => Expr::LocalMethodCall(method_name, params),
                }
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
                Expr::ClassNew(self.hash(name), args)
            }
            Rule::class_ident => Expr::Ident(self.hash(first.as_str())),
            _ => self.build_expr(first).expr,
        };

        for post in inner {
            let post = post.into_inner().next().unwrap();
            let line = &self.as_line_info(&post);

            match post.as_rule() {
                Rule::class_var => {
                    let mut inner = post.into_inner();
                    let mut var_name = self.hash(inner.next().unwrap().as_str());
                    var_name.this_keyword = true;

                    if let Expr::Ident(static_class_name) = &node {
                        if self.static_classes.contains(static_class_name) {
                            node = Expr::StaticGetVar(static_class_name.clone(), var_name);
                            continue; // static early out
                        } else {
                            node = Expr::Ident(static_class_name.clone());
                        }
                    };

                    const LENGTH_VAR: NameHash = hash_const("this.length");

                    if !matches!(node, Expr::StaticGetVar(_, _)) {
                        match var_name {
                            LENGTH_VAR => {
                                node = Expr::NativeMethodCall(NativeMethod::LengthCall, Some(Box::new(expr_node(line, node))), Vec::new());
                            }
                            _ => {
                                node = Expr::ClassGetVar(Box::new(expr_node(line, node)), var_name);
                            }
                        }
                    }
                }
                Rule::class_call => {
                    let mut inner = post.into_inner();

                    let mut fn_name = self.hash(inner.next().unwrap().as_str());
                    fn_name.this_keyword = true;

                    let params: Vec<ExprNode> = inner
                        .next()
                        .unwrap()
                        .into_inner()
                        .map(|p| self.build_expr(p))
                        .collect();

                    const MATH_CLASS: NameHash = hash_const("Math");
                    const RANDOM_FN: NameHash = hash_const("this.random");

                    if let Expr::Ident(static_class_name) = &node {
                        if self.static_classes.contains(static_class_name) {
                            match (static_class_name, &fn_name) {
                                (&MATH_CLASS, &RANDOM_FN) => {
                                    node = Expr::NativeMethodCall(NativeMethod::MathRandom, None, params);
                                }
                                _ => {
                                    node = Expr::StaticMethodCall(
                                        static_class_name.clone(),
                                        fn_name,
                                        params,
                                    );
                                }
                            }
                            continue; // static early out
                        } else {
                            node = Expr::Ident(static_class_name.clone());
                        }
                    };

                    const SUBSTRING_FN: NameHash = hash_const("this.substring");

                    if !matches!(node, Expr::StaticMethodCall(_, _, _)) {
                        match fn_name {
                            SUBSTRING_FN => {
                                node = Expr::NativeMethodCall(NativeMethod::SubstringCall, Some(Box::new(expr_node(line, node))), params);
                            }
                            _ => {
                                node = Expr::ClassMethodCall {
                                    expr: Box::new(expr_node(line, node)),
                                    fn_name,
                                    params,
                                };
                            }
                        }
                    }
                }
                Rule::index => {
                    let idx_pair = post.into_inner().next().unwrap();
                    let idx_expr = self.build_expr(idx_pair);
                    node = Expr::Index(Box::new(expr_node(line, node)), Box::new(idx_expr));
                }
                _ => unreachable!(),
            }
        }
        expr_node(line, node)
    }
}
