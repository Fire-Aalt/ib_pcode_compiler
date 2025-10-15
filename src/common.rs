use include_dir::Dir;

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

pub fn combine_all_paths_at(dir: &Dir, contents_combined: &mut String) {
    for dir in dir.dirs() {
        combine_all_paths_at(dir, contents_combined)
    }
    for file in dir.files() {
        let contents =
            std::str::from_utf8(file.contents()).expect("Should have been able to read the file");
        contents_combined.push_str(contents);
        contents_combined.push('\n');
    }
}

pub fn to_bool_str(string: &str) -> bool {
    !string.is_empty()
}
pub fn to_bool_num(num: f64) -> bool {
    num != 0.0
}
pub fn to_num_bool(bool: bool) -> f64 {
    if bool { 1.0 } else { 0.0 }
}
