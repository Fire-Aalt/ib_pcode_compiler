use crate::ast::AST;
use crate::compiler::compile;
use crate::data::name_hash::with_name_map;
use crate::env::Env;

pub mod common;
pub mod env;
pub mod ast;
pub mod data;
pub mod compiler;

pub fn compile_and_run(code: &str) {
    let ast = compile(code);
    println!("{}", ast);

    let mut env = Env::release();

    run(&ast, &mut env);
    with_name_map(&ast.hash_to_name_map, || {
        println!("Final env: {}", env);
    });
}

pub fn run(ast: &AST, env: &mut Env) {
    ast.traverse(env);
}