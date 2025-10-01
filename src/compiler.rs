use crate::ast::AST;
use crate::compiler::error_print::{print_diagnostic_error, print_parsing_error};
use crate::data::name_hash::with_name_map;
use crate::env::Env;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::fs;
use std::ops::AddAssign;

pub mod error_print;
pub mod errors;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

const COLLECTION: &str = "native_classes/Collection";
const STACK: &str = "native_classes/Stack";
const QUEUE: &str = "native_classes/Queue";

pub fn compile(code: &str, should_panic: bool) -> AST {
    let (program, user_code_start_line) = construct_program_string(code);

    let parsed_result = parse(&program, user_code_start_line, should_panic);

    let ast = build_ast(&program, user_code_start_line, parsed_result);

    with_name_map(&ast.hash_to_name_map, || {
        validate_ast(&ast, should_panic);
    });
    ast
}

fn construct_program_string(code: &str) -> (String, u32) {
    let includes = load_includes();
    let user_code_start_line = includes.lines().count() as u32;

    let mut program = includes.clone();
    program.add_assign(code);
    (program, user_code_start_line)
}

fn load_includes() -> String {
    let include_paths: Vec<&str> = vec![COLLECTION, STACK, QUEUE];

    let mut output = "".to_string();

    for path in include_paths {
        let contents = fs::read_to_string(path).expect("Should have been able to read the file");

        output.add_assign(contents.as_str());
        output.add_assign("\n");
    }
    output
}

fn parse(program: &str, user_code_start_line: u32, should_panic: bool) -> Pair<Rule> {
    match DSLParser::parse(Rule::program, program) {
        Ok(parsed) => parsed,
        Err(err) => {
            print_parsing_error(program, user_code_start_line, err);

            if should_panic {
                panic!()
            } else {
                std::process::exit(0)
            }
        }
    }
    .next()
    .unwrap()
}

fn build_ast(program: &str, user_code_start_line: u32, parsed_result: Pair<Rule>) -> AST {
    let mut ast = AST::new(program.to_string(), user_code_start_line);
    ast.build_ast(parsed_result);
    ast
}

fn validate_ast(ast: &AST, should_panic: bool) {
    let mut env = Env::release();
    let compile_errors = ast.validate(&mut env);

    if !compile_errors.is_empty() {
        for error in &compile_errors {
            print_diagnostic_error(ast, "Compilation", error.clone());
        }

        if should_panic {
            panic!()
        } else {
            std::process::exit(0)
        }
    }
}
