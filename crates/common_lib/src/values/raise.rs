use super::Value;
use crate::{language::LanguageTerm, terms::Raise as RaiseT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Raise<T>
where
    T: LanguageTerm,
{
    pub val: Box<<T as LanguageTerm>::Value>,
    cont_ty: <T as LanguageTerm>::Type,
    exception_ty: <T as LanguageTerm>::Type,
}

impl<T> Raise<T>
where
    T: LanguageTerm,
{
    pub fn new<V1, Ty1, Ty2>(v: V1, cont_ty: Ty1, ex_ty: Ty2) -> Raise<T>
    where
        V1: Into<<T as LanguageTerm>::Value>,
        Ty1: Into<<T as LanguageTerm>::Type>,
        Ty2: Into<<T as LanguageTerm>::Type>,
    {
        Raise {
            val: Box::new(v.into()),
            cont_ty: cont_ty.into(),
            exception_ty: ex_ty.into(),
        }
    }
}

impl<T> Value for Raise<T>
where
    T: LanguageTerm,
{
    type Term = RaiseT<T>;
}

impl<T> From<Raise<T>> for RaiseT<T>
where
    T: LanguageTerm,
{
    fn from(r: Raise<T>) -> RaiseT<T> {
        RaiseT::new(*r.val, r.exception_ty, r.cont_ty)
    }
}

impl<T> fmt::Display for Raise<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raise[{}]({} : {})",
            self.cont_ty, self.val, self.exception_ty
        )
    }
}
