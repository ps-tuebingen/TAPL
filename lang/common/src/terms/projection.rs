use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Projection<T>
where
    T: Term,
{
    term: Box<T>,
    index: usize,
}

impl<T> Term for Projection<T> where T: Term {}

impl<T> fmt::Display for Projection<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).{}", self.term, self.index)
    }
}
