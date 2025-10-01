extern crate core;

use crate::ast::AST;
use crate::compiler::compile;
use crate::compiler::error_print::print_diagnostic_error;
use crate::data::name_hash::with_name_map;
use crate::env::{Env, EnvMode};

pub mod ast;
pub mod common;
pub mod compiler;
pub mod data;
pub mod env;

pub fn compile_release_and_run(code: &str) {
    let ast = compile(code, false);
    //println!("{}", ast);

    let mut env = Env::release();

    run(&ast, &mut env);

    /*    with_name_map(&ast.hash_to_name_map, || {
        println!("Final env: {}", env);
    });*/
}

pub fn run(ast: &AST, env: &mut Env) {
    with_name_map(&ast.hash_to_name_map, || {
        match ast.traverse(env) {
            Ok(_) => {}
            Err(e) => {
                print_diagnostic_error(ast, "Runtime", e);
                match env.mode {
                    EnvMode::Release => std::process::exit(0),
                    EnvMode::Test { .. } => panic!(),
                }
            }
        };
    });
}
