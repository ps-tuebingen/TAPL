use super::Value;
use crate::{language::LanguageTerm, terms::TyLambdaSub as TyLambdaSubT, TypeVar};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TyLambdaSub<T>
where
    T: LanguageTerm,
{
    pub var: TypeVar,
    pub sup: <T as LanguageTerm>::Type,
    pub term: T,
}

impl<T> TyLambdaSub<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Ty>(v: &str, sup: Ty, t: T1) -> TyLambdaSub<T>
    where
        T1: Into<T>,
        Ty: Into<<T as LanguageTerm>::Type>,
    {
        TyLambdaSub {
            var: v.to_owned(),
            sup: sup.into(),
            term: t.into(),
        }
    }
}

impl<T> Value for TyLambdaSub<T>
where
    T: LanguageTerm,
{
    type Term = TyLambdaSubT<T>;
}

impl<T> From<TyLambdaSub<T>> for TyLambdaSubT<T>
where
    T: LanguageTerm,
{
    fn from(tylam: TyLambdaSub<T>) -> TyLambdaSubT<T> {
        TyLambdaSubT::new(&tylam.var, tylam.sup, tylam.term)
    }
}

impl<T> fmt::Display for TyLambdaSub<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:{}.{}", self.var, self.sup, self.term)
    }
}
