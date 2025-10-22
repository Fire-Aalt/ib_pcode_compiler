use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub error_type: ErrorType,
    pub line_info: LineInfo,
    pub message: String,
    pub note: String,
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ErrorType {
    NoReturn,
    OutOfBounds,
    InvalidType,
    Uninitialized,
    Unsupported,
    DuplicateName,
    AssertionFailed,
}

#[derive(Clone, Default)]
pub struct LineInfo {
    pub start_line: u32,
    pub start_col: u16,
    pub end_col: u16,
}

impl Debug for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let raw = match self {
            ErrorType::NoReturn => "No Return",
            ErrorType::OutOfBounds => "Out Of Bounds",
            ErrorType::InvalidType => "Invalid Type",
            ErrorType::Uninitialized => "Uninitialized",
            ErrorType::Unsupported => "Unsupported",
            ErrorType::DuplicateName => "Duplicate Name",
            ErrorType::AssertionFailed => "Assertion Failed",
        };
        write!(f, "{}", raw)
    }
}

impl Debug for LineInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "start line: {}", self.start_line)
    }
}
