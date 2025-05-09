use super::Value;
use crate::{language::LanguageTerm, terms::Right as RightT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Right<T>
where
    T: LanguageTerm,
{
    pub right_val: Box<<T as LanguageTerm>::Value>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Right<T>
where
    T: LanguageTerm,
{
    pub fn new<V, Ty>(val: V, ty: Ty) -> Right<T>
    where
        V: Into<<T as LanguageTerm>::Value>,
        Ty: Into<<T as LanguageTerm>::Type>,
    {
        Right {
            right_val: Box::new(val.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Value for Right<T>
where
    T: LanguageTerm,
{
    type Term = RightT<T>;
}

impl<T> From<Right<T>> for RightT<T>
where
    T: LanguageTerm,
{
    fn from(right: Right<T>) -> RightT<T> {
        RightT::new(*right.right_val, right.ty)
    }
}

impl<T> fmt::Display for Right<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inr({}) as {}", self.right_val, self.ty)
    }
}
