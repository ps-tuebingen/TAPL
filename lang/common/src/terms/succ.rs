use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Succ<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Succ<T> where T: Term {}

impl<T> fmt::Display for Succ<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "succ({})", self.term)
    }
}
