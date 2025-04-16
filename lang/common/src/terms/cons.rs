use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cons<T>
where
    T: LanguageTerm,
{
    head: Box<T>,
    tail: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Cons<T>
where
    T: LanguageTerm,
{
    pub fn new<H, Tl, Typ>(h: H, tl: Tl, ty: Typ) -> Cons<T>
    where
        H: Into<T>,
        Tl: Into<T>,
        Typ: Into<<T as LanguageTerm>::Type>,
    {
        Cons {
            head: Box::new(h.into()),
            tail: Box::new(tl.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Cons<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Cons<T>
where
    T: LanguageTerm,
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

impl<T> SubstType<<T as LanguageTerm>::Type> for Cons<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
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
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
