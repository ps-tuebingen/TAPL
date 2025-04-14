use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Fix<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Fix<T> where T: Term {}

impl<T> fmt::Display for Fix<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fix({})", self.term)
    }
}
