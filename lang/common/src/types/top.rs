use super::Type;
use crate::{subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Top;

impl Type for Top {}

impl<Ty> SubstType<Ty> for Top
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl fmt::Display for Top {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Top")
    }
}
