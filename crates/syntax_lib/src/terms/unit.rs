use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unit<T>
where
    T: Term,
{
    phantom: PhantomData<T>,
}

impl<T> Unit<T>
where
    T: Term,
{
    pub fn new() -> Unit<T> {
        Unit {
            phantom: PhantomData,
        }
    }
}

impl<T> Default for Unit<T>
where
    T: Term,
{
    fn default() -> Unit<T> {
        Unit::new()
    }
}

impl<T> Term for Unit<T> where T: Term {}

impl<T> SubstTerm<T> for Unit<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for Unit<T>
where
    T: Term,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<T> fmt::Display for Unit<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unit")
    }
}
