use crate::{
    TypeVar,
    subst::SubstType,
    terms::Term,
    types::{Type, TypeGroup},
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Untyped<T>
where
    T: Term,
{
    phantom: PhantomData<T>,
}

impl<T> Untyped<T>
where
    T: Term,
{
    pub fn new() -> Untyped<T> {
        Untyped {
            phantom: PhantomData,
        }
    }
}

impl<T> Type for Untyped<T> where T: Term {}

impl<T> fmt::Display for Untyped<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("")
    }
}

impl<T> SubstType<Untyped<T>> for Untyped<T>
where
    T: Term,
{
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &Self) -> Self::Target {
        self
    }
}

impl<T> TypeGroup for Untyped<T> where T: Term {}
