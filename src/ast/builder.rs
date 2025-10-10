use crate::ast::{AST, hash_const};
use crate::compiler::Rule;
use crate::compiler::errors::{compile_error, diagnostic};
use crate::data::ast_nodes::{AssignTarget, Expr, ExprNode, Function, Stmt};
use crate::data::diagnostic::ErrorType;
use crate::data::{NameHash, Validator};
use pest::iterators::{Pair, Pairs};

mod build_expr;
mod build_stmt;

impl AST {
    pub fn build_ast(&mut self, pair: Pair<Rule>, validator: &mut Validator) {
        assert_eq!(pair.as_rule(), Rule::program);

        self.nodes = pair
            .into_inner()
            .map(|inner| self.build_stmt(inner, validator))
            .collect();
    }

    fn build_fn(&mut self, pair: Pair<Rule>, validator: &mut Validator) -> (NameHash, Function) {
        let mut inner = pair.into_inner();

        let fn_name = inner.next().unwrap().as_str();
        let fn_args = self.build_args(&mut inner);
        let mut fn_returns = false;

        let mut fn_body = Vec::new();
        for pair in inner {
            let stmt_node = self.build_stmt(pair, validator);
            if let Stmt::MethodReturn(_) = &stmt_node.stmt {
                fn_returns = true;
            }
            fn_body.push(stmt_node);
        }
        (
            self.hash(fn_name),
            Function {
                args: fn_args,
                body: fn_body,
                returns: fn_returns,
            },
        )
    }

    fn build_args(&mut self, inner: &mut Pairs<Rule>) -> Vec<NameHash> {
        let mut args = Vec::new();
        if let Some(try_inner) = inner.clone().next() {
            if try_inner.as_rule() == Rule::decl_param_list {
                inner.next(); // consume outer
                let inner = try_inner.into_inner();

                for arg in inner {
                    args.push(self.hash(arg.as_str()));
                }
            }
        }
        args
    }

    fn get_assign_target(assignee: ExprNode, validator: &mut Validator) -> AssignTarget {
        match assignee.expr {
            Expr::Ident(name) => AssignTarget::Ident(name),
            Expr::Index(array, index) => AssignTarget::Array(*array, *index),
            _ => {
                compile_error(
                    diagnostic(
                        &assignee.line_info,
                        ErrorType::Unsupported,
                        "can only assign into a local variable or an index expression".into(),
                        "unsupported assign target",
                    ),
                    validator,
                );
                AssignTarget::Ident(hash_const(""))
            }
        }
    }
}
