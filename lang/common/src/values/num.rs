use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Num as NumT, Term},
    types::Type,
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
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
    fn into_lambda<Ty1>(self) -> Result<Lambda<T, Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Lambda Abstraction".to_owned(),
        })
    }

    fn into_raise<Val, Ty1>(self) -> Result<Raise<Val, Ty1, T>, ErrorKind>
    where
        Val: Value<T>,
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Raise".to_owned(),
        })
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
