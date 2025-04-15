use super::Value;
use crate::terms::{Loc as LocT, Term};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Loc {
    loc: usize,
}

impl<T> Value<T> for Loc
where
    T: Term + From<LocT<T>>,
{
    type Term = LocT<T>;
}

impl<T> From<Loc> for LocT<T>
where
    T: Term,
{
    fn from(loc: Loc) -> LocT<T> {
        LocT::new(loc.loc)
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.loc)
    }
}
