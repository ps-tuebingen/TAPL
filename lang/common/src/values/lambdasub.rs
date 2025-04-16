use super::Value;
use crate::{language::LanguageTerm, terms::LambdaSub as LambdaSubT, Var};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LambdaSub<T>
where
    T: LanguageTerm,
{
    pub var: Var,
    pub sup_ty: <T as LanguageTerm>::Type,
    pub term: T,
}

impl<T> Value for LambdaSub<T>
where
    T: LanguageTerm,
{
    type Term = LambdaSubT<T>;
}

impl<T> From<LambdaSub<T>> for LambdaSubT<T>
where
    T: LanguageTerm,
{
    fn from(lam: LambdaSub<T>) -> LambdaSubT<T> {
        LambdaSubT::new(&lam.var, lam.sup_ty, lam.term)
    }
}

impl<T> fmt::Display for LambdaSub<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:{}.{}", self.var, self.sup_ty, self.term)
    }
}
