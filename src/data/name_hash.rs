use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::{fmt, ptr};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct NameHash {
    pub hash: u64,
    pub this_keyword: bool
}

impl NameHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = lookup_name(self) {
            write!(f, "{}", name)
        } else {
            write!(f, "NameHash({})", self.hash)
        }
    }
}

impl PartialEq<NameHash> for &NameHash {
    fn eq(&self, other: &NameHash) -> bool {
        self.hash == other.hash && self.this_keyword == other.this_keyword
    }
}

impl Display for NameHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl Debug for NameHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

thread_local! {
    pub static NAME_MAP_PTR: Cell<*const HashMap<NameHash, String>> = const { Cell::new(ptr::null()) };
}

fn lookup_name<'a>(h: &NameHash) -> Option<&'a str> {
    NAME_MAP_PTR.with(|p| {
        let raw = p.get();
        if raw.is_null() {
            None
        } else {
            // unsafe: raw was stored by us as a pointer to a valid HashMap for this thread
            let map: &HashMap<NameHash, String> = unsafe { &*raw };
            map.get(h).map(|s| s.as_str())
        }
    })
}

/// RAII guard that installs a map pointer for the current thread and restores the previous one on drop.
pub struct NameMapGuard {
    prev: *const HashMap<NameHash, String>,
}

impl NameMapGuard {
    /// Install `map` for this thread, returning a guard that will restore previous pointer on Drop.
    pub fn new(map: &HashMap<NameHash, String>) -> Self {
        let prev = NAME_MAP_PTR.with(|c| {
            let old = c.get();
            c.set(map as *const _);
            old
        });
        NameMapGuard { prev }
    }
}

impl Drop for NameMapGuard {
    fn drop(&mut self) {
        // restore previous pointer using thread_local accessor
        NAME_MAP_PTR.set(self.prev);
    }
}

/// Small helper to run a closure with `map` installed.
pub fn with_name_map<F, R>(map: &HashMap<NameHash, String>, f: F) -> R
where
    F: FnOnce() -> R,
{
    let _guard = NameMapGuard::new(map);
    f()
}
