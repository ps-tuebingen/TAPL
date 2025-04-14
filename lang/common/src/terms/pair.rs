use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Pair<T>
where
    T: Term,
{
    fst: Box<T>,
    snd: Box<T>,
}

impl<T> Term for Pair<T> where T: Term {}

impl<T> fmt::Display for Pair<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {} }}", self.fst, self.snd)
    }
}
