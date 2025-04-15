use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Loc as LocT, Term},
    types::Type,
};
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
