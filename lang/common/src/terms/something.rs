use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Something<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Term for Something<T> where T: Term {}

impl<T, Ty> SubstType<Ty> for Something<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Something {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Something<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something({})", self.term)
    }
}
