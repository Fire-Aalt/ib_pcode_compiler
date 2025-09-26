use crate::ast_nodes::value::Value;
use crate::ast_nodes::Operator;
use crate::env::Env;

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

pub fn num_op(l_val: Value, op: &Operator, r_val: Value) -> Value {
    let l= l_val.as_num();
    let r = r_val.as_num();

    match op {
        Operator::Add => l_val + r_val,
        Operator::Subtract => l_val - r_val,
        Operator::Multiply => l_val * r_val,
        Operator::Divide => l_val / r_val,
        Operator::IntDivide => Value::Number((l_val.as_num() as i64 / r_val.as_num() as i64) as f64),
        Operator::Power => Value::Number(l.powf(r)),
        Operator::Modulo => Value::Number(l % r),
        Operator::Greater => Value::Bool(l > r),
        Operator::Less => Value::Bool(l < r),
        Operator::GreaterEqual => Value::Bool(l >= r),
        Operator::LessEqual => Value::Bool(l <= r),
        Operator::Equal => Value::Bool(l == r),
        Operator::NotEqual => Value::Bool(l != r),
        Operator::And => Value::Bool(to_bool_num(l) && to_bool_num(r)),
        Operator::Or => Value::Bool(to_bool_num(l) || to_bool_num(r)),
    }
}

pub fn str_op(l: &str, op: &Operator, r: &str) -> Value {
    match op {
        Operator::Add => Value::String(String::from(l) + r),
        Operator::Greater => Value::Bool(l > r),
        Operator::Less => Value::Bool(l < r),
        Operator::GreaterEqual => Value::Bool(l >= r),
        Operator::LessEqual => Value::Bool(l <= r),
        Operator::Equal => Value::Bool(l == r),
        Operator::NotEqual => Value::Bool(l != r),
        Operator::And => Value::Bool(to_bool_str(l) && to_bool_str(r)),
        Operator::Or => Value::Bool(to_bool_str(l) || to_bool_str(r)),
        _ => Value::String(String::from("Nan")),
    }
}

pub fn to_bool_str(string: &str) -> bool {
    !string.is_empty()
}
pub fn to_bool_num(num: f64) -> bool {
    num != 0.0
}

pub fn format_val(val: &Value, output: &mut String, env: &Env) {
    match val {
        Value::Number(n) => {
            if n.abs() > 100000000000000000000.0 {
                output.push_str(&format!("{:e}", n));
            }
            else {
                output.push_str(&format!("{}", n));
            }
        },
        Value::String(s) => output.push_str(s.trim()),
        Value::Bool(b) => output.push_str(&b.to_string()),
        Value::Array(id) => {
            for (i, array_val) in env.get_array(id).iter().enumerate() {
                if i > 0 {
                    output.push(',');
                }

                format_val(array_val, output, env);
            }
        },
        Value::Instance(id) => {
            let local = env.get_local_env_at(id);

            output.push_str(local.class_name.as_str());
            output.push_str(": [");

            for (i, (name, val)) in local.scopes.first().unwrap().iter().enumerate() {
                if i > 0 {
                    output.push(',');
                }

                output.push_str(name);
                output.push_str(": ");
                format_val(val, output, env);
            }
            output.push(']');
        },
    }
}