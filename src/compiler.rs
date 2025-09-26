use std::fs;
use std::ops::{Add, AddAssign};
use pest::Parser;
use pest_derive::Parser;
use crate::ast::AST;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

pub fn compile(code: &str) -> AST {
    let includes = load_includes();
    let program = includes.add(code);

    let parsed = DSLParser::parse(Rule::program, program.as_str())
        .expect("parse failed")
        .next()
        .unwrap();

    let mut ast = AST::default();
    ast.build_ast(parsed);
    ast
}


const COLLECTION: &str = "native_classes/Collection";

fn load_includes() -> String {

    let mut contents = fs::read_to_string(COLLECTION)
        .expect("Should have been able to read the file");

    contents.add_assign("\n");

    contents
}
