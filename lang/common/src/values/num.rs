use super::Value;
use crate::{
    errors::ErrorKind,
    terms::{Num as NumT, Term},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Num {
    pub num: i64,
}

impl Num {
    pub fn new(i: i64) -> Num {
        Num { num: i }
    }
}

impl<T> Value<T> for Num
where
    T: Term + From<NumT<T>>,
{
    type Term = NumT<T>;

    fn into_num(self) -> Result<Num, ErrorKind> {
        Ok(self)
    }
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
