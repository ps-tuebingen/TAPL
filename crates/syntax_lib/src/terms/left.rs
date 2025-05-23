use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Left<T>
where
    T: Term,
{
    left_term: Box<T>,
    ty: <T as Term>::Type,
}

impl<T> Left<T>
where
    T: Term,
{
    pub fn new<L, Typ>(left_t: L, ty: Typ) -> Left<T>
    where
        L: Into<T>,
        Typ: Into<<T as Term>::Type>,
    {
        Left {
            left_term: Box::new(left_t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Left<T> where T: Term {}

impl<T> SubstTerm<T> for Left<T>
where
    T: Term,
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

impl<T> SubstType<<T as Term>::Type> for Left<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Left {
            left_term: Box::new(self.left_term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Left<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.left_term, self.ty)
    }
}
