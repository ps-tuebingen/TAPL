use std::fmt;

#[derive(Debug)]
pub struct UndefinedExample {
    ind: usize,
}

impl UndefinedExample {
    #[must_use] pub const fn new(ind: usize) -> Self {
        Self { ind }
    }
}

impl fmt::Display for UndefinedExample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No exmaple with id {}", self.ind)
    }
}

impl std::error::Error for UndefinedExample {}
