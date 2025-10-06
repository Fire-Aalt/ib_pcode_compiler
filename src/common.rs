use std::fs;
use std::path::Path;

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

pub fn get_all_file_paths_at(dir_path: &Path, contents_vec: &mut Vec<String>) {
    if let Ok(dir) = fs::read_dir(dir_path) {
        for entry in dir {
            let entry_path = entry.unwrap().path();
            if entry_path.is_dir() {
                get_all_file_paths_at(entry_path.as_path(), contents_vec)
            } else {
                let contents =
                    fs::read_to_string(entry_path).expect("Should have been able to read the file");
                contents_vec.push(contents);
            }
        }
    }
}

pub fn to_bool_str(string: &str) -> bool {
    !string.is_empty()
}
pub fn to_bool_num(num: f64) -> bool {
    num != 0.0
}
