use crate::ast::AST;
use crate::compiler::error_print::{print_diagnostic_error, print_parsing_error};
use crate::data::Validator;
use crate::data::diagnostic::Diagnostic;
use crate::data::name_hash::with_name_map;
use crate::env::Env;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::collections::HashMap;
use include_dir::{include_dir, Dir};
use crate::common::combine_all_paths_at;

pub mod error_print;
pub mod errors;

static INCLUDE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/include");

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

pub fn compile(code: &str, should_panic: bool) -> AST {
    let (program, user_code_start_line) = construct_program_string(code);

    let parsed_result = parse(&program, user_code_start_line, should_panic);

    let mut validator = Validator {
        validated_functions: HashMap::new(),
        errors: Vec::new(),
        added_errors: 0,
    };

    let ast = build_ast(
        &program,
        user_code_start_line,
        parsed_result,
        &mut validator,
    );
    validate_ast(&ast, &mut validator);

    show_compiler_errors(&ast, should_panic, &validator.errors);

    ast
}

fn construct_program_string(code: &str) -> (String, u32) {
    let includes = load_includes();
    let user_code_start_line = includes.lines().count() as u32;

    let mut program = includes.clone();
    program.push_str(code);
    program.push('\n');
    (program, user_code_start_line)
}

fn load_includes() -> String {
    let mut output = "".to_string();
    combine_all_paths_at(&INCLUDE_DIR, &mut output);
    output
}

fn parse(program: &str, user_code_start_line: u32, should_panic: bool) -> Pair<'_, Rule> {
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

fn build_ast(
    program: &str,
    user_code_start_line: u32,
    parsed_result: Pair<Rule>,
    validator: &mut Validator,
) -> AST {
    let mut ast = AST::new(program.to_string(), user_code_start_line);
    ast.build_ast(parsed_result, validator);
    ast
}

fn validate_ast(ast: &AST, validator: &mut Validator) {
    let mut env = Env::release();

    with_name_map(&ast.hash_to_name_map, || {
        ast.validate(&mut env, validator);
    });
}

fn show_compiler_errors(ast: &AST, should_panic: bool, compile_errors: &Vec<Diagnostic>) {
    if !compile_errors.is_empty() {
        for error in compile_errors {
            print_diagnostic_error(ast, "Compilation", error.clone());
        }

        if should_panic {
            panic!()
        } else {
            std::process::exit(0)
        }
    }
}
