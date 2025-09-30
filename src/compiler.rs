use crate::ast::AST;
use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::ops::AddAssign;
use crate::compiler::error_print::print_parsing_error;

pub mod error_print;
pub mod errors;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

const COLLECTION: &str = "native_classes/Collection";
const STACK: &str = "native_classes/Stack";
const QUEUE: &str = "native_classes/Queue";

pub fn compile(code: &str, should_panic: bool) -> AST {
    let includes = load_includes();
    let user_code_start_line = includes.lines().count() as u32;

    let mut program = includes.clone();
    program.add_assign(code);

    let parsed = match DSLParser::parse(Rule::program, program.as_str()) {
        Ok(parsed) => parsed,
        Err(err) => {
            print_parsing_error(&program, user_code_start_line, err);

            if should_panic {
                panic!()
            } else {
                std::process::exit(0)
            }
        }
    }.next().unwrap();

    let mut ast = AST::new(program.clone(), user_code_start_line);
    ast.build_ast(parsed);
    ast
}

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