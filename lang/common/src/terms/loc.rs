use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Loc {
    loc: usize,
}

impl Term for Loc {}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.loc)
    }
}
