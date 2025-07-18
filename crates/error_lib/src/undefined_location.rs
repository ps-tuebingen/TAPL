use std::fmt;

#[derive(Debug)]
pub struct UndefinedLocation {
    loc: usize,
}

impl UndefinedLocation {
    pub fn new(loc: usize) -> UndefinedLocation {
        UndefinedLocation { loc }
    }
}

impl fmt::Display for UndefinedLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined Location {}", self.loc)
    }
}

impl std::error::Error for UndefinedLocation {}
