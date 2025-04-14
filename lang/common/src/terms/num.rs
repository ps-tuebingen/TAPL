use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug)]
pub struct Num<T>
where
    T: Term,
{
    num: i64,
    phantom: PhantomData<T>,
}

impl<T> Term for Num<T> where T: Term {}

impl<T, Ty> SubstType<Ty> for Num<T>
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

impl<T> fmt::Display for Num<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
