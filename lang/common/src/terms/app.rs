use super::Term;
use std::fmt;

#[derive(Clone, Debug)]
pub struct App<T>
where
    T: Term,
{
    pub fun: Box<T>,
    pub arg: Box<T>,
}

impl<T> Term for App<T> where T: Term {}

impl<T> fmt::Display for App<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) ({})", self.fun, self.arg)
    }
}
