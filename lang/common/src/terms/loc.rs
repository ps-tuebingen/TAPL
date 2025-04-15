use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loc<T>
where
    T: Term,
{
    loc: usize,
    phantom: PhantomData<T>,
}

impl<T> Loc<T>
where
    T: Term,
{
    pub fn new(loc: usize) -> Loc<T> {
        Loc {
            loc,
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Loc<T> where T: Term {}

impl<T> SubstTerm<T> for Loc<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

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
