use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Pred<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Pred<T> where T: Term {}

impl<T> fmt::Display for Pred<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pred({})", self.term)
    }
}
