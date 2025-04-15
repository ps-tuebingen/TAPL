use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Term, True as TrueT},
    types::Type,
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct True;

impl<T> Value<T> for True
where
    T: Term + From<TrueT<T>>,
{
    type Term = TrueT<T>;

    fn into_true(self) -> Result<True, ErrorKind> {
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

impl<T> From<True> for TrueT<T>
where
    T: Term + From<TrueT<T>>,
{
    fn from(_: True) -> TrueT<T> {
        TrueT::new()
    }
}

impl fmt::Display for True {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("true")
    }
}
