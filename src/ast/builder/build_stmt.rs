use crate::ast::builder::get_assign_target;
use crate::ast::AST;
use crate::compiler::Rule;
use crate::data::ast_nodes::{AssignOperator, StmtNode, Class, Constructor, Expr, Stmt};
use pest::iterators::Pair;
use std::collections::HashMap;

impl AST {
    pub fn build_stmt(&mut self, pair: Pair<Rule>) -> StmtNode {
        let line = self.as_line(&pair);
        let stmt = match pair.as_rule() {
            Rule::assign_stmt => {
                let mut inner = pair.into_inner();

                let assignee = self.build_expr(inner.next().unwrap());
                let assign_target = get_assign_target(assignee);

                let op = match inner.next().unwrap().as_rule() {
                    Rule::assign => AssignOperator::Assign,
                    Rule::assign_add => AssignOperator::AssignAdd,
                    Rule::assign_subtract => AssignOperator::AssignSubtract,
                    Rule::assign_multiply => AssignOperator::AssignMultiply,
                    Rule::assign_divide => AssignOperator::AssignDivide,
                    _ => unreachable!(),
                };
                let expr = self.build_expr(inner.next().unwrap());
                Stmt::Assign(assign_target, op, expr)
            }
            Rule::increment_stmt => {
                let mut inner = pair.into_inner();
                let assignee = self.build_expr(inner.next().unwrap());
                let assign_target = get_assign_target(assignee);
                Stmt::Increment(assign_target)
            }
            Rule::decrement_stmt => {
                let mut inner = pair.into_inner();
                let assignee = self.build_expr(inner.next().unwrap());
                let assign_target = get_assign_target(assignee);
                Stmt::Decrement(assign_target)
            }
            Rule::if_stmt => {
                let mut inner = pair.into_inner();
                let cond = self.build_expr(inner.next().unwrap());

                let mut then_branch = Vec::new();
                let mut elifs = Vec::new();
                let mut else_branch = None;

                for p in inner {
                    match p.as_rule() {
                        Rule::elif_clause => {
                            let mut elif_inner = p.into_inner();
                            let elif_cond = self.build_expr(elif_inner.next().unwrap());

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
                let cond = self.build_expr(inner.next().unwrap());
                let body = inner.map(|inner| self.build_stmt(inner)).collect();
                Stmt::While(cond, body)
            }
            Rule::for_loop_stmt => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str();
                let start_num = self.build_expr(inner.next().unwrap());
                let end_num = self.build_expr(inner.next().unwrap());
                let body = inner.map(|inner| self.build_stmt(inner)).collect();
                Stmt::For(self.hash(ident), start_num, end_num, body)
            }
            Rule::loop_until_stmt => {
                let mut inner = pair.into_inner();
                let expr = self.build_expr(inner.next().unwrap());
                let body = inner.map(|inner| self.build_stmt(inner)).collect();
                Stmt::Until(expr, body)
            }
            Rule::input_stmt => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str();
                Stmt::Input(self.hash(ident))
            }
            Rule::output_stmt => {
                let inner = pair.into_inner();
                let body = inner.map(|inner| self.build_expr(inner)).collect();
                Stmt::Output(body)
            }
            Rule::assert_stmt => {
                let mut inner = pair.into_inner();
                let expr = self.build_expr(inner.next().unwrap());
                let expected = self.build_expr(inner.next().unwrap());
                Stmt::Assert(expr, expected)
            }
            Rule::method_decl => {
                let (fn_name, function) = self.build_fn(pair);
                self.add_function(fn_name.clone(), function);

                Stmt::FunctionDeclaration(fn_name)
            }
            Rule::class_decl => {
                let mut inner = pair.into_inner();

                let class_name_hash = self.hash(inner.next().unwrap().as_str());
                let args = self.build_args(&mut inner);

                let mut constructors = Vec::new();
                let mut functions = HashMap::new();

                for stmt in inner {
                    match stmt.as_rule() {
                        Rule::class_constructor_stmt => {
                            let mut inner = stmt.into_inner();

                            let var_name = inner.next().unwrap().as_str();
                            let expr = self.build_expr(inner.next().unwrap());

                            constructors.push((self.hash(var_name), expr));
                        }
                        Rule::class_function => {
                            let (fn_name, function) = self.build_fn(stmt);
                            functions.insert(fn_name.clone(), function);
                        }
                        _ => unreachable!(),
                    }
                }

                self.add_class(class_name_hash.clone(), Class {
                    functions,
                    constructor: Constructor { constructors, args },
                });

                Stmt::ClassDeclaration(class_name_hash)
            }
            Rule::expr_stmt => {
                let mut inner = pair.into_inner();
                let expr = self.build_expr(inner.next().unwrap());
                Stmt::Expr(expr)
            }
            Rule::method_return => {
                let mut inner = pair.into_inner();
                Stmt::MethodReturn(self.build_expr(inner.next().unwrap()))
            }
            Rule::EOI => Stmt::EOI,
            _ => unreachable!(),
        };
        StmtNode { line_info: line, stmt }
    }
}