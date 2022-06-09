use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd)]
pub struct Result {
    pub dist: f32,
    pub id: i32,
}

// https://github.com/rust-lang/rust-clippy/issues/6219
#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Result {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.partial_cmp(&other.dist).unwrap()
    }
}

impl Eq for Result {}
