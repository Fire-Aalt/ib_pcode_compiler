use crate::ast::AST;
use crate::compiler::Rule;
use crate::data::ast_nodes::{AssignTarget, AstNode, Expr, Function};
use crate::data::NameHash;
use pest::iterators::{Pair, Pairs};

mod build_stmt;
mod build_expr;

impl AST {
    pub fn build_ast(&mut self, pair: Pair<Rule>) {
        assert_eq!(pair.as_rule(), Rule::program);

        self.nodes = pair
            .into_inner()
            .map(|inner| self.build_stmt(inner))
            .collect();
    }

    fn build_fn(&mut self, pair: Pair<Rule>) -> (NameHash, Function) {
        let mut inner = pair.into_inner();

        let fn_name = inner.next().unwrap().as_str();
        let fn_args = self.build_args(&mut inner);

        let fn_body: Vec<AstNode> = inner.map(|inner| self.build_stmt(inner)).collect();
        (self.hash(fn_name), Function { args: fn_args, body: fn_body } )
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
}

fn get_assign_target(assignee: Expr) -> AssignTarget {
    match assignee {
        Expr::Ident(name) => {
            AssignTarget::Ident(name)
        }
        Expr::Index(array, index) => {
            AssignTarget::Array(*array, *index)
        }
        _ => unreachable!(),
    }
}




