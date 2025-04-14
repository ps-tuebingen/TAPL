use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Try<T>
where
    T: Term,
{
    term: Box<T>,
    handler: Box<T>,
}

impl<T> Term for Try<T> where T: Term {}

impl<T> fmt::Display for Try<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} with {{ {} }}", self.term, self.handler)
    }
}
