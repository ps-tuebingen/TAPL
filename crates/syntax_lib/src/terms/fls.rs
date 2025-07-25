use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct False<T>
where
    T: Term,
{
    phantom: PhantomData<T>,
}

impl<T> False<T>
where
    T: Term,
{
    pub fn new() -> False<T> {
        False {
            phantom: PhantomData,
        }
    }
}

impl<T> Default for False<T>
where
    T: Term,
{
    fn default() -> False<T> {
        False::new()
    }
}

impl<T> Term for False<T> where T: Term {}

impl<T> SubstTerm<T> for False<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for False<T>
where
    T: Term,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        False {
            phantom: PhantomData,
        }
        .into()
    }
}

impl<T> fmt::Display for False<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("false")
    }
}
