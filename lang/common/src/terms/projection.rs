use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Projection<T>
where
    T: Term,
{
    term: Box<T>,
    index: usize,
}

impl<T> Term for Projection<T> where T: Term {}

impl<T> SubstTerm<T> for Projection<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Projection {
            term: Box::new(self.term.subst(v, t)),
            index: self.index,
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Projection<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Projection {
            term: Box::new(self.term.subst_type(v, ty)),
            index: self.index,
        }
        .into()
    }
}

impl<T> fmt::Display for Projection<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).{}", self.term, self.index)
    }
}
