use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cons<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub head: Box<T>,
    pub tail: Box<T>,
    pub ty: Ty,
}

impl<T, Ty> Cons<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<H, Tl, Typ>(h: H, tl: Tl, ty: Typ) -> Cons<T, Ty>
    where
        H: Into<T>,
        Tl: Into<T>,
        Typ: Into<Ty>,
    {
        Cons {
            head: Box::new(h.into()),
            tail: Box::new(tl.into()),
            ty: ty.into(),
        }
    }
}

impl<T, Ty> Term for Cons<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for Cons<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
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

impl<T, Ty> SubstType<Ty> for Cons<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Cons {
            head: Box::new(self.head.subst_type(v, ty)),
            tail: Box::new(self.tail.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Cons<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
