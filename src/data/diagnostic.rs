use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct Diagnostic {
    pub error_type: ErrorType,
    pub line_info: LineInfo,
    pub message: String,
}

pub enum ErrorType {
    NoReturn,
    OutOfBounds,
    InvalidType,
    Uninitialized,
}

#[derive(Clone)]
pub struct LineInfo {
    pub start_line: i32,
    pub end_line: i32,
    pub start_pos: u16,
    pub end_pos: u16,
}

impl Debug for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let raw = match self {
            ErrorType::NoReturn     => "No Return",
            ErrorType::OutOfBounds  => "Out Of Bounds",
            ErrorType::InvalidType  => "Invalid Type",
            ErrorType::Uninitialized => "Uninitialized",
        };
        write!(f, "{}", raw)
    }
}

impl Debug for LineInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "start line: {}", self.start_line)
    }
}