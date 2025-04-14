use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Deref<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Deref<T> where T: Term {}

impl<T> fmt::Display for Deref<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "!{}", self.term)
    }
}
