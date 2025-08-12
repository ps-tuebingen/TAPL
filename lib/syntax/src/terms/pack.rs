use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pack<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub inner_ty: Ty,
    pub term: Box<T>,
    pub outer_ty: Ty,
}

impl<T, Ty> Pack<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<Ty1, Ty2, T1>(inner: Ty1, t: T1, outer: Ty2) -> Pack<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
        T1: Into<T>,
    {
        Pack {
            inner_ty: inner.into(),
            term: Box::new(t.into()),
            outer_ty: outer.into(),
        }
    }
}

impl<T, Ty> Term for Pack<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for Pack<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Pack {
            inner_ty: self.inner_ty,
            term: Box::new(self.term.subst(v, t)),
            outer_ty: self.outer_ty,
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Pack<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Pack {
            inner_ty: self.inner_ty.subst_type(v, ty),
            term: Box::new(self.term.subst_type(v, ty)),
            outer_ty: self.outer_ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Pack<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{*({}),{}}} as {}",
            self.inner_ty, self.term, self.outer_ty
        )
    }
}
