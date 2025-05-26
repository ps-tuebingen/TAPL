use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Left<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub left_term: Box<T>,
    pub ty: Ty,
}

impl<T, Ty> Left<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<L, Typ>(left_t: L, ty: Typ) -> Left<T, Ty>
    where
        L: Into<T>,
        Typ: Into<Ty>,
    {
        Left {
            left_term: Box::new(left_t.into()),
            ty: ty.into(),
        }
    }
}

impl<T, Ty> Term for Left<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for Left<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Left {
            left_term: Box::new(self.left_term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Left<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Left {
            left_term: Box::new(self.left_term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Left<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.left_term, self.ty)
    }
}
