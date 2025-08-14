use std::fmt;

#[derive(Debug)]
pub struct UndefinedExample {
    ind: usize,
}

impl UndefinedExample {
    pub fn new(ind: usize) -> UndefinedExample {
        UndefinedExample { ind }
    }
}

impl fmt::Display for UndefinedExample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No exmaple with id {}", self.ind)
    }
}

impl std::error::Error for UndefinedExample {}
