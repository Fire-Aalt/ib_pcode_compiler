
pub mod utils {
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
}