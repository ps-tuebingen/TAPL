use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct TryWithVal<T>
where
    T: Term,
{
    term: Box<T>,
    handler: Box<T>,
}

impl<T> Term for TryWithVal<T> where T: Term {}

impl<T> fmt::Display for TryWithVal<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} catch {{ {} }}", self.term, self.handler)
    }
}
