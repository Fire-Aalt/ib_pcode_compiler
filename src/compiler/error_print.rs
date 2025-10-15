use crate::ast::{write_output, AST};
use crate::compiler::Rule;
use crate::data::diagnostic::Diagnostic;
use pest::error::{Error, ErrorVariant, InputLocation};
use std::cmp::max;

#[cfg(target_arch = "wasm32")]
const RED: &str = "<span style=\"color:red;\">";
#[cfg(not(target_arch = "wasm32"))]
const RED: &str = "\x1b[31m";

#[cfg(target_arch = "wasm32")]
const RESET: &str = "</span>";
#[cfg(not(target_arch = "wasm32"))]
const RESET: &str = "\x1b[0m";

struct ErrorLine {
    pub user_start_line: isize,
    pub start_line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub fn print_diagnostic_error(ast: &AST, error_category: &str, diagnostic: Diagnostic) {
    let start_line = diagnostic.line_info.start_line as usize;

    let error_line = ErrorLine {
        user_start_line: start_line as isize - ast.user_code_start_line as isize,
        start_line,
        start_col: diagnostic.line_info.start_col as usize,
        end_col: diagnostic.line_info.end_col as usize,
    };
    
    let msg = &mut String::new();

    msg.push_str(RED);
    msg.push_str(format!("{} error: {}\n", error_category, diagnostic.message).as_str());
    push_line_info(&ast.source, diagnostic.note.as_str(), &error_line, msg);
    msg.push_str(RESET);
    print_to_console(msg);
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
    let (_end_line, end_col) = line_col_of(end_byte);

    let positives = match &err.variant {
        ErrorVariant::ParsingError {
            positives,
            negatives: _,
        } => positives.clone(),
        _ => Vec::new(),
    };

    let user_start_line = start_line as isize - user_code_start_line as isize;

    let error_line = ErrorLine {
        user_start_line,
        start_line: start_line - 1,
        start_col,
        end_col: end_col + 1,
    };


    let msg = &mut String::new();

    msg.push_str(RED);
    msg.push_str("Parsing error\n");
    push_line_info(program, "", &error_line, msg);
    msg.push_str(format!("Expected grammar: {:?}\n", positives).as_str());
    msg.push_str(RESET);
    print_to_console(msg);
}

fn push_line_info(source: &str, note: &str, info: &ErrorLine, msg: &mut String) {
    let lines: Vec<&str> = source.lines().collect();

    msg.push_str(format!("At line: {}\n", info.user_start_line).as_str());

    if let Some(line_text) = lines.get(info.start_line - 1) {
        let indent_len = info.user_start_line.to_string().chars().count();

        let mut ident = String::new();
        for _ in 0..indent_len {
            ident.push(' ');
        }
        
        msg.push_str(format!("{} | \n", ident).as_str());
        msg.push_str(format!("{} | {}\n", info.user_start_line, line_text).as_str());

        let mut underline = String::new();
        for _ in 1..info.start_col {
            underline.push(' ');
        }

        let width = max(1, info.end_col.saturating_sub(info.start_col));

        msg.push_str(format!("{} | ", ident).as_str());
        for _ in 0..width {
            underline.push('^');
        }
        msg.push_str(format!("{} {}\n", underline, note).as_str());
    }
}

fn print_to_console(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        write_output(msg);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        eprintln!("{}", msg);
    }
}
