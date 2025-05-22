use super::Value;
use crate::{language::LanguageTerm, terms::Pack as PackT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pack<T>
where
    T: LanguageTerm,
{
    pub inner_ty: <T as LanguageTerm>::Type,
    pub val: Box<<T as LanguageTerm>::Value>,
    pub outer_ty: <T as LanguageTerm>::Type,
}

impl<T> Pack<T>
where
    T: LanguageTerm,
{
    pub fn new<Ty1, V, Ty2>(inner: Ty1, v: V, outer: Ty2) -> Pack<T>
    where
        Ty1: Into<<T as LanguageTerm>::Type>,
        Ty2: Into<<T as LanguageTerm>::Type>,
        V: Into<<T as LanguageTerm>::Value>,
    {
        Pack {
            inner_ty: inner.into(),
            val: Box::new(v.into()),
            outer_ty: outer.into(),
        }
    }
}

impl<T> Value for Pack<T>
where
    T: LanguageTerm,
{
    type Term = PackT<T>;
}

impl<T> From<Pack<T>> for PackT<T>
where
    T: LanguageTerm,
{
    fn from(pack: Pack<T>) -> PackT<T> {
        PackT::new(pack.inner_ty, *pack.val, pack.outer_ty)
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
            self.inner_ty, self.val, self.outer_ty
        )
    }
}
