use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug)]
pub struct Loc<T>
where
    T: Term,
{
    loc: usize,
    phantom: PhantomData<T>,
}

impl<T> Term for Loc<T> where T: Term {}

impl<T, Ty> SubstType<Ty> for Loc<T>
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

impl<T> fmt::Display for Loc<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.loc)
    }
}
