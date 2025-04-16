use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pack<T>
where
    T: LanguageTerm,
{
    inner_ty: <T as LanguageTerm>::Type,
    term: Box<T>,
    outer_ty: <T as LanguageTerm>::Type,
}

impl<T> Pack<T>
where
    T: LanguageTerm,
{
    pub fn new<Ty1, Ty2, T1>(inner: Ty1, t: T1, outer: Ty2) -> Pack<T>
    where
        Ty1: Into<<T as LanguageTerm>::Type>,
        Ty2: Into<<T as LanguageTerm>::Type>,
        T1: Into<T>,
    {
        Pack {
            inner_ty: inner.into(),
            term: Box::new(t.into()),
            outer_ty: outer.into(),
        }
    }
}

impl<T> Term for Pack<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Pack<T>
where
    T: LanguageTerm,
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

impl<T> SubstType<<T as LanguageTerm>::Type> for Pack<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Pack {
            inner_ty: self.inner_ty.subst_type(v, ty),
            term: Box::new(self.term.subst_type(v, ty)),
            outer_ty: self.outer_ty.subst_type(v, ty),
        }
        .into()
    }
}
impl<T> fmt::Display for Pack<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{*{},{}}} as {}",
            self.inner_ty, self.term, self.outer_ty
        )
    }
}
