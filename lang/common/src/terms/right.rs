use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Right<T>
where
    T: LanguageTerm,
{
    right_term: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Right<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Ty1>(right_t: T1, ty: Ty1) -> Right<T>
    where
        T1: Into<T>,
        Ty1: Into<<T as LanguageTerm>::Type>,
    {
        Right {
            right_term: Box::new(right_t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Right<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Right<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Right {
            right_term: Box::new(self.right_term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Right<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Right {
            right_term: Box::new(self.right_term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Right<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.right_term, self.ty)
    }
}
