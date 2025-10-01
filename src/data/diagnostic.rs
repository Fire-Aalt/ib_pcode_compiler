use crate::data::{Validator, Value};
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub error_type: ErrorType,
    pub line_info: LineInfo,
    pub message: String,
    pub note: String,
}

#[derive(Clone)]
pub enum ErrorType {
    NoReturn,
    OutOfBounds,
    InvalidType,
    Uninitialized,
    Unsupported
}

#[derive(Clone, Default)]
pub struct LineInfo {
    pub start_line: u32,
    pub end_line: u32,
    pub start_col: u16,
    pub end_col: u16,
}

impl LineInfo {
    pub fn compile_error(&self, error_type: ErrorType, message: &str, note: &str, validator: &mut Validator) -> Result<(), Diagnostic> {
        let err = self.diagnostic(error_type, message, note);
        validator.errors.push(err.clone());
        Err(err)
    }

    pub fn runtime_error(&self, error_type: ErrorType, message: &str) -> Result<Value, Diagnostic> {
        Err(self.diagnostic(error_type, message, ""))
    }

    pub fn runtime_option_error(&self, error_type: ErrorType, message: &str) -> Result<Option<Value>, Diagnostic> {
        Err(self.diagnostic(error_type, message, ""))
    }

    pub fn diagnostic(&self, error_type: ErrorType, message: &str, note: &str) -> Diagnostic {
        Diagnostic {
            error_type,
            line_info: self.clone(),
            message: message.to_string(),
            note: note.to_string(),
        }
    }
}

impl Debug for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let raw = match self {
            ErrorType::NoReturn => "No Return",
            ErrorType::OutOfBounds => "Out Of Bounds",
            ErrorType::InvalidType => "Invalid Type",
            ErrorType::Uninitialized => "Uninitialized",
           ErrorType::Unsupported => "Unsupported",
        };
        write!(f, "{}", raw)
    }
}

impl Debug for LineInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "start line: {}", self.start_line)
    }
}