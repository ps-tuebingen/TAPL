use super::Value;
use crate::{kinds::Kind, language::LanguageTerm, terms::TyLambda as TyLambdaT, TypeVar};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TyLambda<T>
where
    T: LanguageTerm,
{
    var: TypeVar,
    annot: Kind,
    term: T,
}

impl<T> Value for TyLambda<T>
where
    T: LanguageTerm,
{
    type Term = TyLambdaT<T>;
}

impl<T> From<TyLambda<T>> for TyLambdaT<T>
where
    T: LanguageTerm,
{
    fn from(tylam: TyLambda<T>) -> TyLambdaT<T> {
        TyLambdaT::new(&tylam.var, tylam.annot, tylam.term)
    }
}

impl<T> fmt::Display for TyLambda<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}.{}", self.var, self.term)
    }
}
