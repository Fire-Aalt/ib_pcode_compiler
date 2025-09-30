use crate::data::{NameHash, Validator, Value};
use crate::data::diagnostic::{Diagnostic, ErrorType, LineInfo};

pub fn compile_error(line_info: &LineInfo, mut diagnostic: Diagnostic, validator: &mut Validator) -> Result<(), Diagnostic> {
    diagnostic.line_info = line_info.clone();
    validator.errors.push(diagnostic.clone());
    Err(diagnostic)
}

pub fn runtime_error(line_info: &LineInfo, mut diagnostic: Diagnostic) -> Result<Value, Diagnostic> {
    diagnostic.line_info = line_info.clone();
    Err(diagnostic)
}

pub fn no_return_error(fn_name: &NameHash, class_name: &NameHash) -> Diagnostic {
    Diagnostic {
        line_info: LineInfo::default(),
        error_type: ErrorType::NoReturn,
        message: format!(
            "not all code paths return for function `{}` in class `{}`",
            fn_name, class_name
        ),
        note: "expected to return a value".to_string(),
    }
}

pub fn uninitialized_var_error(var_name: &NameHash) -> Diagnostic {
    Diagnostic {
        line_info: LineInfo::default(),
        error_type: ErrorType::Uninitialized,
        message: format!("cannot find variable `{}` in this scope", var_name),
        note: "not found in this scope".to_string(),
    }
}

pub fn out_of_bounds_error(index: i64, length: usize) -> Diagnostic {
    Diagnostic {
        line_info: LineInfo::default(),
        error_type: ErrorType::OutOfBounds,
        message: format!("Index {} is out of bounds {}", index, length),
        note: "".to_string(),
    }
}