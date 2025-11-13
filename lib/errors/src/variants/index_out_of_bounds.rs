use std::fmt;

#[derive(Debug)]
pub struct IndexOutOfBounds {
    tried: usize,
    len: usize,
}

impl IndexOutOfBounds {
    #[must_use] pub const fn new(tried: usize, len: usize) -> Self {
        Self { tried, len }
    }
}

impl fmt::Display for IndexOutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Index {} is out of bounds (length is {})",
            self.tried, self.len
        )
    }
}

impl std::error::Error for IndexOutOfBounds {}
