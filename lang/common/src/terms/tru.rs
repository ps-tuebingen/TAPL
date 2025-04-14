use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug)]
pub struct True<T>
where
    T: Term,
{
    phantom: PhantomData<T>,
}

impl<T> Term for True<T> where T: Term {}

impl<T, Ty> SubstType<Ty> for True<T>
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

impl<T> fmt::Display for True<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("true")
    }
}
