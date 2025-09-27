#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
pub struct NameHash {
    pub hash: u64,
    pub this_keyword: bool
}