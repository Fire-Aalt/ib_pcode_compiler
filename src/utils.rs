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

pub fn to_num_bool(data: bool) -> f64 {
    match data {
        true => 1.0,
        false => 0.0,
    }
}

pub fn to_string_bool(data: bool) -> String {
    match data {
        true => String::from("true"),
        false => String::from("false"),
    }
}