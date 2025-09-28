use std::fmt::{Debug, Formatter};
use crate::data::ast_nodes::ExprNode;
use crate::data::Value;

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
    pub start_line: u32,
    pub end_line: u32,
    pub start_col: u16,
    pub end_col: u16,
}

impl LineInfo {
    pub fn valid_error(&self, error_type: ErrorType, message: &str) -> Result<(), Diagnostic> {
        Err(self.diagnostic(error_type, message))
    }

    pub fn error(&self, error_type: ErrorType, message: &str) -> Result<Value, Diagnostic> {
        Err(self.diagnostic(error_type, message))
    }

    pub fn option_error(&self, error_type: ErrorType, message: &str) -> Result<Option<Value>, Diagnostic> {
        Err(self.diagnostic(error_type, message))
    }

    pub fn diagnostic(&self, error_type: ErrorType, message: &str) -> Diagnostic {
        Diagnostic {
            error_type,
            line_info: self.clone(),
            message: message.to_string(),
        }
    }
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