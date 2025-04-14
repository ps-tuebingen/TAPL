use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Fst<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Fst<T> where T: Term {}

impl<T> fmt::Display for Fst<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.0", self.term)
    }
}
