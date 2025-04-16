use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TyApp<T>
where
    T: LanguageTerm,
{
    fun: Box<T>,
    arg: <T as LanguageTerm>::Type,
}

impl<T> Term for TyApp<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for TyApp<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        TyApp {
            fun: Box::new(self.fun.subst(v, t)),
            arg: self.arg,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for TyApp<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        TyApp {
            fun: Box::new(self.fun.subst_type(v, ty)),
            arg: self.arg.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for TyApp<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})[{}]", self.fun, self.arg)
    }
}
