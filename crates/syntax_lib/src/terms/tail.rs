use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tail<T, Ty>
where
    T: Term,
{
    term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Tail<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<T1, Ty1>(t: T1, ty: Ty1) -> Tail<T, Ty>
    where
        T1: Into<T>,
        Ty: Into<Ty>,
    {
        Tail {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T, Ty> Term for Tail<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for Tail<T, Ty>
where
    T: Term,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Tail {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Tail<T, Ty>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Tail {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Tail<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tail[{}]({})", self.term, self.term)
    }
}
