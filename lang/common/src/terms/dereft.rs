use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Deref<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Deref<T> where T: Term {}

impl<T> SubstTerm<T> for Deref<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Deref {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Deref<T>
where
    Ty: Type,
    T: Term + SubstType<Ty, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Deref {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Deref<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "!{}", self.term)
    }
}
