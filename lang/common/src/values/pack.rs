use super::Value;
use crate::{language::LanguageTerm, terms::Pack as PackT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pack<T>
where
    T: LanguageTerm,
{
    inner_ty: <T as LanguageTerm>::Type,
    val: Box<<T as LanguageTerm>::Value>,
    outer_ty: <T as LanguageTerm>::Type,
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
