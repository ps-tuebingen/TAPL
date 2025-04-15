use super::Value;
use crate::terms::{Term, True as TrueT};

pub struct True;

impl<T> Value<T> for True
where
    T: Term,
{
    type Term = TrueT<T>;
}

impl<T> From<True> for TrueT<T>
where
    T: Term,
{
    fn from(_: True) -> TrueT<T> {
        TrueT::new()
    }
}
