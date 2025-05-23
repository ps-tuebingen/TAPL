use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable<T>
where
    T: Term,
{
    var: Var,
    phantom: PhantomData<T>,
}

impl<T: Term> Variable<T> {
    pub fn new(v: &str) -> Variable<T> {
        Variable {
            var: v.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Variable<T> where T: Term {}

impl<T> SubstTerm<T> for Variable<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            t.clone()
        } else {
            self.into()
        }
    }
}

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
