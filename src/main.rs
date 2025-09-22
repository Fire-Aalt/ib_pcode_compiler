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

        output "=== Simple Calculations ==="

output "Adding 1...10 = " , 1+2+3+4+5+6+7+8+9+10

output "10 Factorial = " , 1*2*3*4*5*6*7*8*9*10

output "Fractions = 1/2 + 1/4 + 1/5 = " , 1/2 + 1/4 + 1/5

output "Pythagoras = 3^2 + 4^2 = 5^2 = " , 3*3 + 4*4 , " and " , 5*5

output "Big Numbers = one trillion ^ 2 = " , 1000000000000 * 1000000000000

output "Easier big numbers = " , 2e12 * 3e12

output "10307 is not prime = " , 10307 / 11 , " * " , 11

output "15% of 12345 = " , 15/100*12345

output "Incorrect calculation = " , 1234567890 * 1234567890

output "Another error = " , 1/2 + 1/3 + 1/6

output "One more problem = " , 0.1+0.1+0.1+0.1+0.1+0.1+0.1+0.1

output "And another problem = " , 3.2 - 0.3

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