use super::Value;
use crate::terms::{Num as NumT, Term};
use std::fmt;

#[derive(Debug)]
pub struct Num {
    num: i64,
}

impl<T> Value<T> for Num
where
    T: Term + From<NumT<T>>,
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

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
