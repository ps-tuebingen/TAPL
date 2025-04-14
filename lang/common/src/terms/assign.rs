use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Assign<T>
where
    T: Term,
{
    lhs: Box<T>,
    rhs: Box<T>,
}

impl<T> Term for Assign<T> where T: Term {}

impl<T> fmt::Display for Assign<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} := {}", self.lhs, self.rhs)
    }
}
