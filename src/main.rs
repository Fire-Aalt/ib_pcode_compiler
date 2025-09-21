use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use crate::ast::AST;
use crate::ast_nodes::Value;

pub mod utils;
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

        XR -= "Me"

        // comment
        loop I from -81 to 10
            if XR >= -99 then
                XR -= 1.5
            end if

            output XR
        end loop
    "#;

    let mut ast = compile(code);
    println!("{:#?}", ast.method_map);
    println!("{}", ast);

    run(&mut ast);
    println!("Final env: {:?}", ast.env);
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


fn run(ast: &mut AST) {
    ast.traverse();
}