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
// --- LOGIC ---
// The computer can perform more complex LOGIC
// by using BOOLEAN OPERATORS like AND and OR.
// For example, consider the problem of inputting a user name and password.
// Both the account name and the password must be correct.
// So the NAME AND PASSWORD must be correct.
// This example program shows how to use AND / OR correctly.

 NAME = input("Type your user name")
 PASSWORD = input("Type your password")

 if  NAME = "bozo"  AND  PASSWORD = "clown"  then
    output "Correct!"
 end if

 if  NAME = "einstein"  AND  PASSWORD = "e=mc2"  then
    output "Correct!"
 end if

 if  NAME = "guest"  OR  NAME = "trial"  then
    output "You will be logged in as a GUEST"
 end if


    "#;

    let mut ast = compile(code);
    println!("{:#?}", ast.method_map);
    println!("{}", ast);

    let mut env = Env::new();

    run(&mut ast, &mut env);
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


fn run(ast: &mut AST, env: &mut Env) {
    ast.traverse(env);
}