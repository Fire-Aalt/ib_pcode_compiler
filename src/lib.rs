extern crate core;

use crate::ast::AST;
use crate::compiler::compile;
use crate::data::diagnostic::Diagnostic;
use crate::data::name_hash::with_name_map;
use crate::env::Env;

pub mod common;
pub mod env;
pub mod ast;
pub mod data;
pub mod compiler;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub fn compile_and_run(code: &str) {
    let ast = compile(code);
    println!("{}", ast);

    let mut env = Env::release();

    run(&ast, &mut env);
    with_name_map(&ast.hash_to_name_map, || {
        println!("Final env: {}", env);
    });
}

pub fn run(ast: &AST, env: &mut Env) {
    with_name_map(&ast.hash_to_name_map, || {
        match ast.traverse(env) {
            Ok(_) => {}
            Err(e) => print_diagnostic(ast, e)
        };
    });
}

pub fn print_diagnostic(ast: &AST, diagnostic: Diagnostic) {
    let start_line = (ast.start_line as i32 + diagnostic.line_info.start_line) as usize;
    let end_line = (ast.start_line as i32 + diagnostic.line_info.end_line) as usize;
    let start_col = diagnostic.line_info.start_pos as usize;
    let end_col = diagnostic.line_info.end_pos as usize;

    let lines: Vec<&str> = ast.source.lines().collect();

    eprintln!("{}", RED);
    eprintln!("Runtime {:?} error: {}", diagnostic.error_type, diagnostic.message);

    if diagnostic.line_info.end_line != diagnostic.line_info.start_line {
        eprint!("At lines: {}-{}", diagnostic.line_info.start_line, diagnostic.line_info.end_line);
    } else {
        eprint!("At line: {}", diagnostic.line_info.start_line);
    }

    eprint!(", ");

    if diagnostic.line_info.end_pos - 1 != diagnostic.line_info.start_pos {
        eprint!("characters: {}-{}", start_col, end_col);
    } else {
        eprint!("character: {}", start_col);
    }
    eprintln!();

    if let Some(line_text) = lines.get(start_line - 1) {
        eprintln!("{}", line_text);

        let mut underline = String::new();
        for _ in 1..start_col {
            underline.push(' ');
        }

        let width = if start_line == end_line {
            std::cmp::max(1, end_col.saturating_sub(start_col))
        } else {
            line_text.chars().count().saturating_sub(start_col - 1)
        };

        for _ in 0..width {
            underline.push('^');
        }
        eprintln!("{}", underline);
    }

    if end_line > start_line {
        if let Some(last_line) = lines.get(end_line - 1) {
            eprintln!("...");
            eprintln!("{}", last_line);

            let mut underline = String::new();
            for _ in 1..=end_col {
                underline.push('^');
            }
            eprintln!("{}", underline);
        }
    }
    eprintln!("{}", RESET);
}