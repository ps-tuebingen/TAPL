use super::Value;
use crate::terms::{Num as NumT, Term};

pub struct Num {
    num: i64,
}

impl<T> Value<T> for Num
where
    T: Term,
{
    type Term = NumT<T>;
}

impl<T> From<Num> for NumT<T>
where
    T: Term,
{
    fn from(n: Num) -> NumT<T> {
        NumT::new(n.num)
    }
}
