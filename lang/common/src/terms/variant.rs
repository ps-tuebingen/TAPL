use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    Label, TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<T, Ty>
where
    T: Term,
    Ty: Type,
{
    label: Label,
    term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Variant<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<T1, Ty1>(lb: &str, t: T1, ty: Ty1) -> Variant<T, Ty>
    where
        T1: Into<T>,
        Ty1: Into<Ty>,
    {
        Variant {
            label: lb.to_owned(),
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T, Ty> Term for Variant<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for Variant<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
    Ty: Type,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Variant {
            label: self.label,
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Variant<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Variant {
            label: self.label,
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Variant<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} = {}> as {}", self.label, self.term, self.ty)
    }
}
