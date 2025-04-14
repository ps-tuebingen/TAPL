use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Head<T, Ty>
where
    T: Term,
    Ty: Type,
{
    term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Term for Head<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for Head<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
    Ty: Type,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Head {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Head<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Head {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Head<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "head[{}]({})", self.ty, self.term)
    }
}
