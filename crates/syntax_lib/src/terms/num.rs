use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Nat,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Num<T>
where
    T: Term,
{
    num: i64,
    phantom: PhantomData<T>,
}

impl<T> Num<T>
where
    T: Term,
{
    pub fn new(num: i64) -> Num<T> {
        Num {
            num,
            phantom: PhantomData,
        }
    }
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

impl<T> SubstType<<T as Term>::Type> for Num<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &<T as Term>::Type) -> Self::Target {
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
