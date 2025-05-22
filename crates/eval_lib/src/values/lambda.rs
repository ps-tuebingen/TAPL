use super::Value;
use crate::{language::LanguageTerm, terms::Lambda as LambdaT, Var};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lambda<T>
where
    T: LanguageTerm,
{
    pub var: Var,
    pub annot: <T as LanguageTerm>::Type,
    pub body: T,
}

impl<T> Lambda<T>
where
    T: LanguageTerm,
{
    pub fn new<Ty1, T1>(v: &str, ty: Ty1, bd: T1) -> Lambda<T>
    where
        T1: Into<T>,
        Ty1: Into<<T as LanguageTerm>::Type>,
    {
        Lambda {
            var: v.to_owned(),
            annot: ty.into(),
            body: bd.into(),
        }
    }
}

impl<T> Value for Lambda<T>
where
    T: LanguageTerm,
{
    type Term = LambdaT<T>;
}

impl<T> From<Lambda<T>> for LambdaT<T>
where
    T: LanguageTerm,
{
    fn from(lam: Lambda<T>) -> LambdaT<T> {
        LambdaT::new(&lam.var, lam.annot, lam.body)
    }
}

impl<T> fmt::Display for Lambda<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty_str = self.annot.to_string();
        if ty_str.is_empty() {
            write!(f, "\\{}.{}", self.var, self.body)
        } else {
            write!(f, "\\{}:{}.({})", self.var, ty_str, self.body)
        }
    }
}
