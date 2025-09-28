use crate::ast::AST;
use crate::common::fix_quotes_plain;
use crate::compiler::Rule;
use crate::data::ast_nodes::{Expr, ExprNode, Operator, UnaryOp};
use crate::data::Value;
use pest::iterators::Pair;
use crate::data::diagnostic::LineInfo;

fn expr_node(line: LineInfo, expr: Expr) -> ExprNode {
    ExprNode { line_info: line, expr }
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
                    let line = self.as_sub_line(&op_pair);

                    let right = self.build_expr(inner.next().unwrap());
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
                    left = expr_node(line, Expr::BinOp(Box::new(left), op, Box::new(right)));
                }
                left
            }
            Rule::pow => {
                let mut inner = pair.into_inner();
                let left = self.build_expr(inner.next().unwrap());
                if let Some(op_pair) = inner.next() {
                    let line = self.as_sub_line(&op_pair);
                    let right = self.build_expr(inner.next().unwrap());

                    expr_node(
                        line,
                        Expr::BinOp(Box::new(left), Operator::Power, Box::new(right)),
                    )
                } else {
                    left
                }
            }
            Rule::unary => {
                let mut parts: Vec<_> = pair.into_inner().collect();
                let mut expr = self.build_expr(parts.pop().unwrap());

                while let Some(op_pair) = parts.pop() {
                    let line = self.as_sub_line(&op_pair);

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
        let line = self.as_sub_line(&first);

        let mut node = match first.as_rule() {
            Rule::ident => {
                expr_node(line, Expr::Ident(self.hash(first.as_str()))) //TODO: validate the var was declared
            }
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
            Rule::array => {
                let inner = first.into_inner();
                let data = inner.map(|inner| self.build_expr(inner)).collect();
                expr_node(line, Expr::Array(data))
            }
            Rule::method_call => {
                let mut inner = first.into_inner();

                let method_name = inner.next().unwrap().as_str();
                let args = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|inner| self.build_expr(inner))
                    .collect();
                expr_node(line, Expr::MethodCall(self.hash(method_name), args))
            }
            Rule::input_call => {
                let mut inner = first.into_inner();
                let text = self.build_expr(inner.next().unwrap());
                expr_node(line, Expr::Input(Box::new(text)))
            }
            Rule::div_call => {
                let mut inner = first.into_inner();
                let left = self.build_expr(inner.next().unwrap());
                let right = self.build_expr(inner.next().unwrap());
                expr_node(line, Expr::Div(Box::new(left), Box::new(right)))
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
            let line = self.as_sub_line(&post);

            match post.as_rule() {
                Rule::substring_call => {
                    let mut inner = post.into_inner();
                    let start = self.build_expr(inner.next().unwrap());
                    let end = self.build_expr(inner.next().unwrap());
                    node = expr_node(line, Expr::SubstringCall {
                        expr: Box::new(node),
                        start: Box::new(start),
                        end: Box::new(end),
                    });
                }
                Rule::call => {
                    let mut inner = post.into_inner();

                    let fn_name = inner.next().unwrap().as_str();
                    let params = inner
                        .next()
                        .unwrap()
                        .into_inner()
                        .map(|p| self.build_expr(p))
                        .collect();

                    let mut fn_name_hash = self.hash(fn_name);
                    fn_name_hash.this_keyword = true;

                    node = expr_node(line, Expr::Call {
                        expr: Box::new(node),
                        fn_name: fn_name_hash,
                        params,
                    });
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
