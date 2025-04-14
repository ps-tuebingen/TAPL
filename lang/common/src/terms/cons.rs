use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Cons<T, Ty>
where
    T: Term,
    Ty: Type,
{
    head: Box<T>,
    tail: Box<T>,
    ty: Ty,
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
        write!(f, "cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
