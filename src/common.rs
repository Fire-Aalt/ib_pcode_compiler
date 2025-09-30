use crate::data::ast_nodes::Operand;
use crate::data::Value;

pub fn fix_quotes_plain(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(&next) = chars.peek() {
                if next == '"' {
                    chars.next();
                    out.push('"');
                    continue;
                }
            }
            out.push(c);
        } else if c == '"' {
            continue;
        } else {
            out.push(c);
        }
    }
    out
}

pub fn num_op(l_val: Value, op: &Operand, r_val: Value) -> Value {
    let l= l_val.as_num();
    let r = r_val.as_num();

    match op {
        Operand::Add => l_val + r_val,
        Operand::Subtract => l_val - r_val,
        Operand::Multiply => l_val * r_val,
        Operand::Divide => l_val / r_val,
        Operand::IntDivide => Value::Number((l_val.as_num() as i64 / r_val.as_num() as i64) as f64),
        Operand::Power => Value::Number(l.powf(r)),
        Operand::Modulo => Value::Number(l % r),
        Operand::Greater => Value::Bool(l > r),
        Operand::Less => Value::Bool(l < r),
        Operand::GreaterEqual => Value::Bool(l >= r),
        Operand::LessEqual => Value::Bool(l <= r),
        Operand::Equal => Value::Bool(l == r),
        Operand::NotEqual => Value::Bool(l != r),
        Operand::And => Value::Bool(to_bool_num(l) && to_bool_num(r)),
        Operand::Or => Value::Bool(to_bool_num(l) || to_bool_num(r)),
    }
}

pub fn str_op(l: &str, op: &Operand, r: &str) -> Value {
    match op {
        Operand::Add => Value::String(String::from(l) + r),
        Operand::Greater => Value::Bool(l > r),
        Operand::Less => Value::Bool(l < r),
        Operand::GreaterEqual => Value::Bool(l >= r),
        Operand::LessEqual => Value::Bool(l <= r),
        Operand::Equal => Value::Bool(l == r),
        Operand::NotEqual => Value::Bool(l != r),
        Operand::And => Value::Bool(to_bool_str(l) && to_bool_str(r)),
        Operand::Or => Value::Bool(to_bool_str(l) || to_bool_str(r)),
        _ => Value::String(String::from("Nan")),
    }
}

pub fn to_bool_str(string: &str) -> bool {
    !string.is_empty()
}
pub fn to_bool_num(num: f64) -> bool {
    num != 0.0
}

