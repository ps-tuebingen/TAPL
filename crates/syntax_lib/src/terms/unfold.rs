use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unfold<T, Ty>
where
    T: Term,
    Ty: Type,
{
    ty: Ty,
    term: Box<T>,
}

impl<T, Ty> Unfold<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<T1, Ty1>(ty: Ty1, t: T1) -> Unfold<T, Ty>
    where
        T1: Into<T>,
        Ty1: Into<Ty>,
    {
        Unfold {
            ty: ty.into(),
            term: Box::new(t.into()),
        }
    }
}

impl<T, Ty> Term for Unfold<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for Unfold<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Unfold {
            ty: self.ty,
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Unfold<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Unfold {
            ty: self.ty.subst_type(v, ty),
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Unfold<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unfold[{}]({})", self.ty, self.term)
    }
}
