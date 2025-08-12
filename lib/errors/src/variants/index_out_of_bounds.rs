use std::fmt;

#[derive(Debug)]
pub struct IndexOutOfBounds {
    tried: usize,
    len: usize,
}

impl IndexOutOfBounds {
    pub fn new(tried: usize, len: usize) -> IndexOutOfBounds {
        IndexOutOfBounds { tried, len }
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
