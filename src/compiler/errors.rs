use crate::data::ast_nodes::Operand;
use crate::data::diagnostic::{Diagnostic, ErrorType, LineInfo};
use crate::data::{NameHash, Validator, Value};

pub fn compile_error(diagnostic: Diagnostic, validator: &mut Validator) {
    validator.errors.push(diagnostic.clone());
    validator.added_errors += 1;
}

pub fn diagnostic(
    line_info: &LineInfo,
    error_type: ErrorType,
    message: String,
    note: &str,
) -> Diagnostic {
    Diagnostic {
        line_info: line_info.clone(),
        error_type,
        message,
        note: note.to_string(),
    }
}

pub fn no_return_error(
    line_info: &LineInfo,
    fn_name: &NameHash,
    class_name: &NameHash,
) -> Diagnostic {
    Diagnostic {
        line_info: line_info.clone(),
        error_type: ErrorType::NoReturn,
        message: format!(
            "not all code paths return for function `{}` in class `{}`",
            fn_name, class_name
        ),
        note: "expected to return a value".to_string(),
    }
}

pub fn no_public_var_error(
    line_info: &LineInfo,
    var_name: &NameHash,
    class_name: &NameHash,
) -> Diagnostic {
    Diagnostic {
        line_info: line_info.clone(),
        error_type: ErrorType::NoReturn,
        message: format!(
            "public variable `{}` was not found in class `{}` ",
            var_name, class_name
        ),
        note: "undefined public variable".to_string(),
    }
}

pub fn undefined_fn_in_class_error(
    line_info: &LineInfo,
    class_name: &NameHash,
    fn_name: &NameHash,
) -> Diagnostic {
    Diagnostic {
        line_info: line_info.clone(),
        error_type: ErrorType::Uninitialized,
        message: format!("undefined function `{}` in class `{}`", fn_name, class_name),
        note: "undefined function".to_string(),
    }
}

pub fn out_of_bounds_error(line_info: &LineInfo, index: i64, length: usize) -> Diagnostic {
    Diagnostic {
        line_info: line_info.clone(),
        error_type: ErrorType::OutOfBounds,
        message: format!("index `{}` is out of bounds `{}`", index, length),
        note: "tries to access invalid memory".to_string(),
    }
}

pub fn invalid_number_of_params_error(
    line_info: &LineInfo,
    provided_number: usize,
    expected: String,
) -> Diagnostic {
    Diagnostic {
        line_info: line_info.clone(),
        error_type: ErrorType::OutOfBounds,
        message: format!(
            "provided number of parameters `{}` is not the same as requested `{}`",
            provided_number, expected
        ),
        note: "incorrect number of params".to_string(),
    }
}

pub fn invalid_type_call_error(
    line_info: &LineInfo,
    method: &str,
    val: &Value,
    supported: &str,
    note: &str,
) -> Diagnostic {
    Diagnostic {
        line_info: line_info.clone(),
        error_type: ErrorType::InvalidType,
        message: format!(
            "{} used on `{}`. Only {} are supported",
            method, val, supported
        ),
        note: note.to_string(),
    }
}

pub fn unsupported_operand_error(
    line_info: &LineInfo,
    left: &Value,
    op: &Operand,
    right: &Value,
) -> Diagnostic {
    Diagnostic {
        line_info: line_info.clone(),
        error_type: ErrorType::Unsupported,
        message: format!(
            "unsupported operand `{}` for types `{}` and `{}`",
            op.error_fmt(),
            left.error_fmt(),
            right.error_fmt()
        ),
        note: "results in undefined behavior".to_string(),
    }
}
