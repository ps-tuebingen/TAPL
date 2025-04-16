use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    Label, TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<T>
where
    T: LanguageTerm,
{
    label: Label,
    term: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Variant<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Ty1>(lb: &str, t: T1, ty: Ty1) -> Variant<T>
    where
        T1: Into<T>,
        Ty1: Into<<T as LanguageTerm>::Type>,
    {
        Variant {
            label: lb.to_owned(),
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Variant<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Variant<T>
where
    T: LanguageTerm,
    Self: Into<T>,
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

impl<T> SubstType<<T as LanguageTerm>::Type> for Variant<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Variant {
            label: self.label,
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Variant<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} = {}> as {}", self.label, self.term, self.ty)
    }
}
