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

        method Sussy(A, B)
            if A == 0 then
                return 0
            end if

            output "Recursion:", A, B
            A = A - 1
            return Sussy(A, B)
        end method

        method Meth(A)
            A = A - 1

            output A
            if A == 0 then
                return 0
            end if

            return Meth(A)
        end method


        XR = 4.5
        A = 45

        output A + "Haha"
        output A + 14

        Sussy(A, XR)
        output Meth(45)

        XR /= "sdad"
        output XR
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