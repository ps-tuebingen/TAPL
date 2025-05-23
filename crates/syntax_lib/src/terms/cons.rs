use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cons<T>
where
    T: Term,
{
    head: Box<T>,
    tail: Box<T>,
    ty: <T as Term>::Type,
}

impl<T> Cons<T>
where
    T: Term,
{
    pub fn new<H, Tl, Typ>(h: H, tl: Tl, ty: Typ) -> Cons<T>
    where
        H: Into<T>,
        Tl: Into<T>,
        Typ: Into<<T as Term>::Type>,
    {
        Cons {
            head: Box::new(h.into()),
            tail: Box::new(tl.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Cons<T> where T: Term {}

impl<T> SubstTerm<T> for Cons<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Cons {
            head: Box::new(self.head.subst(v, t)),
            tail: Box::new(self.tail.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Cons<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Cons {
            head: Box::new(self.head.subst_type(v, ty)),
            tail: Box::new(self.tail.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Cons<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
