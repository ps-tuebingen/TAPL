use std::fmt;

#[derive(Debug)]
pub struct EmptyCase;

impl fmt::Display for EmptyCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot have zero patterns in match")
    }
}

impl std::error::Error for EmptyCase {}
