use crate::ast::{AST, hash_const};
use crate::common::fix_quotes_plain;
use crate::compiler::Rule;
use crate::data::ast_nodes::{Expr, ExprNode, NativeMethod, Operand, UnaryOp};
use crate::data::diagnostic::LineInfo;
use crate::data::{NameHash, Value};
use pest::iterators::Pair;

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
            Rule::ident => Expr::Var(self.hash(first.as_str())),
            Rule::number => Expr::Data(Value::Number(first.as_str().parse().unwrap())),
            Rule::string => Expr::Data(Value::String(fix_quotes_plain(first.as_str()))),
            Rule::bool => Expr::Data(Value::Bool(first.as_str().parse().unwrap())),
            Rule::undefined => Expr::Data(Value::Undefined),
            Rule::array => {
                let inner = first.into_inner();
                let data = inner.map(|inner| self.build_expr(inner)).collect();
                Expr::ArrayNew(data)
            }
            Rule::method_call => {
                let mut inner = first.into_inner();

                let fn_rule = inner.next().unwrap();
                let fn_line = self.as_line_info(&fn_rule);
                let method_name = self.hash(fn_rule.as_str());

                let params: Vec<ExprNode> = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|inner| self.build_expr(inner))
                    .collect();

                const INPUT: NameHash = hash_const("input");

                match method_name {
                    INPUT => Expr::NativeFunctionCall(NativeMethod::Input, None, fn_line, params),
                    _ => Expr::LocalFunctionCall(method_name, params),
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
            Rule::class_ident => Expr::Var(self.hash(first.as_str())),
            _ => self.build_expr(first).expr,
        };

        for post in inner {
            let post = post.into_inner().next().unwrap();
            let post_line = &self.as_line_info(&post);

            match post.as_rule() {
                Rule::class_var => {
                    let mut inner = post.into_inner();
                    let var_name = self.hash_with_this_keyword(inner.next().unwrap().as_str());

                    if let Expr::Var(static_class_name) = &node {
                        if self.static_classes.contains(static_class_name) {
                            node = Expr::StaticGetVar(
                                post_line.clone(),
                                static_class_name.clone(),
                                var_name,
                            );
                            continue; // static early out
                        } else {
                            node = Expr::Var(static_class_name.clone());
                        }
                    };

                    const LENGTH_VAR: NameHash = hash_const("this.length");

                    if !matches!(node, Expr::StaticGetVar(_, _, _)) {
                        match var_name {
                            LENGTH_VAR => {
                                node = Expr::NativeFunctionCall(
                                    NativeMethod::LengthCall,
                                    Some(Box::new(expr_node(line, node))),
                                    post_line.clone(),
                                    Vec::new(),
                                );
                            }
                            _ => {
                                node = Expr::ClassGetVar(
                                    Box::new(expr_node(line, node)),
                                    post_line.clone(),
                                    var_name,
                                );
                            }
                        }
                    }
                }
                Rule::class_call => {
                    let mut inner = post.into_inner();
                    let fn_name = self.hash_with_this_keyword(inner.next().unwrap().as_str());

                    let params: Vec<ExprNode> = inner
                        .next()
                        .unwrap()
                        .into_inner()
                        .map(|p| self.build_expr(p))
                        .collect();

                    const MATH_CLASS: NameHash = hash_const("Math");
                    const RANDOM_FN: NameHash = hash_const("this.random");

                    if let Expr::Var(static_class_name) = &node {
                        if self.static_classes.contains(static_class_name) {
                            match (static_class_name, &fn_name) {
                                (&MATH_CLASS, &RANDOM_FN) => {
                                    node = Expr::NativeFunctionCall(
                                        NativeMethod::MathRandom,
                                        None,
                                        post_line.clone(),
                                        params,
                                    );
                                }
                                _ => {
                                    node = Expr::StaticFunctionCall(
                                        post_line.clone(),
                                        static_class_name.clone(),
                                        fn_name,
                                        params,
                                    );
                                }
                            }
                            continue; // static early out
                        } else {
                            node = Expr::Var(static_class_name.clone());
                        }
                    };

                    const SUBSTRING_FN: NameHash = hash_const("this.substring");

                    if !matches!(node, Expr::StaticFunctionCall(_, _, _, _)) {
                        match fn_name {
                            SUBSTRING_FN => {
                                node = Expr::NativeFunctionCall(
                                    NativeMethod::SubstringCall,
                                    Some(Box::new(expr_node(line, node))),
                                    post_line.clone(),
                                    params,
                                );
                            }
                            _ => {
                                node = Expr::ClassFunctionCall {
                                    expr: Box::new(expr_node(line, node)),
                                    fn_line: post_line.clone(),
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

fn expr_node(line: &LineInfo, expr: Expr) -> ExprNode {
    ExprNode {
        line_info: line.clone(),
        expr,
    }
}
