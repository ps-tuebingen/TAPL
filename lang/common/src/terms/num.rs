use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
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

impl<T> SubstTerm<T> for Num<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

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
