use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Something<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Something<T> where T: Term {}

impl<T> fmt::Display for Something<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something({})", self.term)
    }
}
