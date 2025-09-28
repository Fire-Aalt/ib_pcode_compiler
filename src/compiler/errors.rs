use pest::error::{Error, ErrorVariant, InputLocation};
use crate::ast::AST;
use crate::data::diagnostic::Diagnostic;
use crate::compiler::Rule;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub fn print_diagnostic_error(ast: &AST, error_category: &str, diagnostic: Diagnostic) {
    let start_line = diagnostic.line_info.start_line as usize;
    let end_line = diagnostic.line_info.end_line as usize;
    let start_col = diagnostic.line_info.start_col as usize;
    let end_col = diagnostic.line_info.end_col as usize;

    let user_start_line = start_line - ast.user_code_start_line as usize;
    let user_end_line = end_line - ast.user_code_start_line as usize;

    eprint!("{}", RED);
    eprintln!("{} {:?} error: {}", error_category, diagnostic.error_type, diagnostic.message);
    print_line_info(&ast.source, user_start_line, user_end_line, start_line, end_line, start_col, end_col);
    eprint!("{}", RESET);
}

pub fn print_parsing_error(program: &str, user_code_start_line: u32, err: Error<Rule>) {
    let (start_byte, end_byte) = match &err.location {
        InputLocation::Pos(p) => (*p, *p),
        InputLocation::Span((s, e)) => (*s, *e),
    };

    // clamp to source bounds to be defensive
    let src_len = program.len();
    let start_byte = start_byte.min(src_len);
    let end_byte = end_byte.min(src_len);

    // helper to compute (line, col) 1-based from a byte index
    let line_col_of = |byte_idx: usize| -> (usize, usize) {
        let prefix = &program[..byte_idx];
        let line = prefix.lines().count() + 1;

        let last_nl_pos = prefix.rfind('\n').map(|p| p + 1).unwrap_or(0);
        let col = program[last_nl_pos..byte_idx].chars().count() + 1;

        (line, col)
    };

    let (start_line, start_col) = line_col_of(start_byte);
    let (end_line, end_col) = line_col_of(end_byte);

    let positives = match &err.variant {
        ErrorVariant::ParsingError { positives, negatives: _ } => positives.clone(),
        _ => Vec::new(),
    };

    let user_start_line = start_line - user_code_start_line as usize;
    let user_end_line = end_line - user_code_start_line as usize;

    eprint!("{}", RED);
    eprintln!("Parsing error");
    print_line_info(program, user_start_line, user_end_line, start_line - 1, end_line - 1, start_col, end_col + 1);
    eprintln!("Expected grammar: {:?}", positives);
    eprint!("{}", RESET);
}

fn print_line_info(source: &str, user_start_line: usize, user_end_line: usize, start_line: usize, end_line: usize, start_col: usize, end_col: usize) {
    let lines: Vec<&str> = source.lines().collect();

    if user_end_line != user_start_line {
        eprint!("At lines: {}-{}", user_start_line, user_end_line);
    } else {
        eprint!("At line: {}", user_start_line);
    }

    eprint!(", ");

    if end_col - 1 != start_col {
        eprint!("characters: {}-{}", start_col, end_col);
    } else {
        eprint!("character: {}", start_col);
    }
    eprintln!();

    if let Some(line_text) = lines.get(start_line - 1) {
        eprintln!(" |");
        eprintln!(" | {}", line_text);

        let mut underline = String::new();
        for _ in 1..start_col {
            underline.push(' ');
        }

        let width = if start_line == end_line {
            std::cmp::max(1, end_col.saturating_sub(start_col))
        } else {
            line_text.chars().count().saturating_sub(start_col - 1)
        };

        eprint!(" | ");
        for _ in 0..width {
            underline.push('^');
        }
        eprintln!("{}", underline);
    }

    if end_line > start_line {
        if let Some(last_line) = lines.get(user_end_line - 1) {
            eprintln!("...");
            eprintln!("{}", last_line);

            let mut underline = String::new();
            for _ in 1..=end_col {
                underline.push('^');
            }
            eprintln!("{}", underline);
        }
    }
}