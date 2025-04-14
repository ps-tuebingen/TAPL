use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Snd<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Snd<T> where T: Term {}

impl<T> fmt::Display for Snd<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).1", self.term)
    }
}
