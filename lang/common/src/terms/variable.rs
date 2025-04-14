use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar, Var};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug)]
pub struct Variable<T>
where
    T: Term,
{
    var: Var,
    phantom: PhantomData<T>,
}

impl<T> Term for Variable<T> where T: Term {}

impl<T, Ty> SubstType<Ty> for Variable<T>
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

impl<T> fmt::Display for Variable<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}
