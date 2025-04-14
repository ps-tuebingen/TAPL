use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Ref<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Ref<T> where T: Term {}

impl<T> fmt::Display for Ref<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ref({})", self.term)
    }
}
