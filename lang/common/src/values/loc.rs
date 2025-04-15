use super::Value;
use crate::terms::{Loc as LocT, Term};

pub struct Loc {
    loc: usize,
}

impl<T> Value<T> for Loc
where
    T: Term,
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
