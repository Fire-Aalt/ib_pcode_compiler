use crate::ast_nodes::value::Value;
use crate::ast_nodes::Operator;

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

pub fn num_op(l: f64, op: &Operator, r: f64) -> Value {
    Value::Number(match op {
        Operator::Add => l + r,
        Operator::Subtract => l - r,
        Operator::Multiply => l * r,
        Operator::Divide => l / r,
        Operator::Power => l.powf(r),
        Operator::Modulo => l % r,
        Operator::Greater => to_num_bool(l > r),
        Operator::Less => to_num_bool(l < r),
        Operator::GreaterEqual => to_num_bool(l >= r),
        Operator::LessEqual => to_num_bool(l <= r),
        Operator::Equal => to_num_bool(l == r),
        Operator::NotEqual => to_num_bool(l != r),
        Operator::And => to_num_bool(l != 0.0 && r != 0.0),
        Operator::Or => to_num_bool(l != 0.0 || r != 0.0),
    })
}

pub fn str_op(l: &str, op: &Operator, r: &str) -> Value {
    Value::String(match op {
        Operator::Add => String::from(l) + r,
        Operator::Greater => to_str_bool(l > r),
        Operator::Less => to_str_bool(l < r),
        Operator::GreaterEqual => to_str_bool(l >= r),
        Operator::LessEqual => to_str_bool(l <= r),
        Operator::Equal => to_str_bool(l == r),
        Operator::NotEqual => to_str_bool(l != r),
        Operator::And => to_str_bool(to_bool_str(l) && to_bool_str(r)),
        Operator::Or => to_str_bool(to_bool_str(l) || to_bool_str(r)),
        _ => String::from("Nan"),
    })
}

pub fn to_bool_str(string: &str) -> bool {
    !string.is_empty()
}

pub fn to_num_bool(data: bool) -> f64 {
    match data {
        true => 1.0,
        false => 0.0,
    }
}

pub fn to_str_bool(data: bool) -> String {
    match data {
        true => String::from("true"),
        false => String::from("false"),
    }
}