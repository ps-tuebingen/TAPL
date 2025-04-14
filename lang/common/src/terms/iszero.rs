use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct IsZero<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for IsZero<T> where T: Term {}

impl<T> fmt::Display for IsZero<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iszero({})", self.term)
    }
}
