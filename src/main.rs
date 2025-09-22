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
// === Mice ============
//  JavaScript has loops, including for loops
//  like Java and Basic and other languages.
//  This example uses loops for a poem.

loop A from 1 to 2
   output "Three blind mice"
end loop

loop B from 3 to 4
   output "See how they run"
end loop

output "They all ran up to the farmer's wife"
output "She cut off their tails with a carving knife"
output "Did you ever see such a sight in your life, as"

C = 5
loop while C < 20
   output "Three blind mice"
   C = C*2
end loop


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