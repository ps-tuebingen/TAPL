use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Num {
    num: i64,
}

impl Term for Num {}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
