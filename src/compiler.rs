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
const STACK: &str = "native_classes/Stack";
const QUEUE: &str = "native_classes/Queue";


fn load_includes() -> String {
    let include_paths: Vec<&str> = vec![COLLECTION, STACK, QUEUE];

    let mut output = "".to_string();

    for path in include_paths {
        let contents = fs::read_to_string(path)
            .expect("Should have been able to read the file");

        output.add_assign(contents.as_str());
        output.add_assign("\n");
    }

    output
}
