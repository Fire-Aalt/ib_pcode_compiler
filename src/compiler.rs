use pest::Parser;
use pest_derive::Parser;
use crate::ast::AST;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

pub fn compile(code: &str) -> AST {
    let parsed = DSLParser::parse(Rule::program, code)
        .expect("parse failed")
        .next()
        .unwrap();

    let mut ast = AST::default();
    ast.build_ast(parsed);
    ast
}
