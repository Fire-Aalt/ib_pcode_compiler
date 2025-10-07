use crate::data::NameHash;

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

const fn fnv1a_from(s: &str, start: usize) -> u64 {
    let bytes = s.as_bytes();
    let mut hash = FNV_OFFSET;
    let mut i = start;

    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        i += 1;
    }
    hash
}

const fn starts_with_this(s: &str) -> bool {
    let pat = b"this.";
    let s_bytes = s.as_bytes();
    if s_bytes.len() < pat.len() {
        return false;
    }
    let mut i = 0;
    while i < pat.len() {
        if s_bytes[i] != pat[i] {
            return false;
        }
        i += 1;
    }
    true
}

pub const fn hash(s: &str) -> NameHash {
    if starts_with_this(s) {
        let h = fnv1a_from(s, 5);
        NameHash::new(h, true)
    } else {
        let h = fnv1a_from(s, 0);
        NameHash::new(h, false)
    }
}