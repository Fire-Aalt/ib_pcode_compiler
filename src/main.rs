use crate::ast::AST;
use crate::env::Env;
use pest::Parser;
use pest_derive::Parser;

pub mod utils;
pub mod env;
pub mod ast;
pub mod ast_nodes;
pub mod ast_builder;
pub mod ast_evaluator;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

fn main() {
    let code = r#"

X = 150

output X / 427493

    "#;

    let ast = compile(code);
    println!("{:#?}", ast.method_map);
    println!("{}", ast);

    let mut env = Env::release();

    run(&ast, &mut env);
    println!("Final env: {:?}", env);
}

fn compile(code: &str) -> AST {
    let parsed = DSLParser::parse(Rule::program, code)
        .expect("parse failed")
        .next()
        .unwrap();

    let mut ast = AST::default();
    ast.build_ast(parsed);
    ast
}


fn run(ast: &AST, env: &mut Env) {
    ast.traverse(env);
}