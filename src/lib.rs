use crate::ast::AST;
use crate::compiler::compile;
use crate::env::{Env};

pub mod common;
pub mod env;
pub mod ast;
pub mod ast_nodes;
pub mod ast_builder;
pub mod ast_evaluator;
pub mod compiler;

pub fn compile_and_run(code: &str) {
    let ast = compile(code);
    println!("{:#?}", ast.function_map);
    println!("{:#?}", ast.class_map);
    println!("{}", ast);

    let mut env = Env::release();

    run(&ast, &mut env);
    println!("Final env: {:?}", env);
}

pub fn run(ast: &AST, env: &mut Env) {
    ast.traverse(env);
}