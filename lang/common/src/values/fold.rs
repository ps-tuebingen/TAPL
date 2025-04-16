use super::Value;
use crate::{language::LanguageTerm, terms::Fold as FoldT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Fold<T>
where
    T: LanguageTerm,
{
    ty: <T as LanguageTerm>::Type,
    val: Box<<T as LanguageTerm>::Value>,
}

impl<T> Value for Fold<T>
where
    T: LanguageTerm,
    Self: Into<FoldT<T>>,
{
    type Term = FoldT<T>;
}

impl<T> From<Fold<T>> for FoldT<T>
where
    T: LanguageTerm,
{
    fn from(fld: Fold<T>) -> FoldT<T> {
        FoldT::new(*fld.val, fld.ty)
    }
}

impl<T> fmt::Display for Fold<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold[{}]({})", self.ty, self.val)
    }
}
