use crate::ast::AST;
use crate::compiler::Rule;
use crate::data::diagnostic::Diagnostic;
use pest::error::{Error, ErrorVariant, InputLocation};

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

struct ErrorLine {
    pub user_start_line: usize,
    pub user_end_line: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub fn print_diagnostic_error(ast: &AST, error_category: &str, diagnostic: Diagnostic) {
    let start_line = diagnostic.line_info.start_line as usize;
    let end_line = diagnostic.line_info.end_line as usize;

    let error_line = ErrorLine {
        user_start_line: start_line - ast.user_code_start_line as usize,
        user_end_line: end_line - ast.user_code_start_line as usize,
        start_line,
        end_line,
        start_col: diagnostic.line_info.start_col as usize,
        end_col: diagnostic.line_info.end_col as usize,
    };

    eprint!("{}", RED);
    eprintln!("{} error: {}", error_category, diagnostic.message);
    print_line_info(&ast.source, diagnostic.note.as_str(), &error_line);
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

    let error_line = ErrorLine {
        user_start_line,
        user_end_line,
        start_line: start_line - 1,
        end_line: end_line - 1,
        start_col,
        end_col: end_col + 1,
    };

    eprint!("{}", RED);
    eprintln!("Parsing error");
    print_line_info(program, "", &error_line);
    eprintln!("Expected grammar: {:?}", positives);
    eprint!("{}", RESET);
}

fn print_line_info(source: &str, note: &str, info: &ErrorLine) {
    let lines: Vec<&str> = source.lines().collect();

    if info.user_end_line != info.user_start_line {
        eprint!("At lines: {}-{}", info.user_start_line, info.user_end_line);
    } else {
        eprint!("At line: {}", info.user_start_line);
    }
    eprintln!();

    if let Some(line_text) = lines.get(info.start_line - 1) {
        let indent_len = info.user_start_line.to_string().chars().count();

        let mut ident = String::new();
        for _ in 0..indent_len {
            ident.push(' ');
        }
        eprintln!("{} | ", ident);
        eprintln!("{} | {}", info.user_start_line, line_text);

        let mut underline = String::new();
        for _ in 1..info.start_col {
            underline.push(' ');
        }

        let width = if info.start_line == info.end_line {
            std::cmp::max(1, info.end_col.saturating_sub(info.start_col))
        } else {
            line_text.chars().count().saturating_sub(info.start_col - 1)
        };

        eprint!("{} | ", ident);
        for _ in 0..width {
            underline.push('^');
        }
        eprintln!("{} {}", underline, note);
    }

    if info.end_line > info.start_line {
        if let Some(last_line) = lines.get(info.user_end_line - 1) {
            eprintln!("...");
            eprintln!("{}", last_line);

            let mut underline = String::new();
            for _ in 1..=info.end_col {
                underline.push('^');
            }
            eprintln!("{}", underline);
        }
    }
}