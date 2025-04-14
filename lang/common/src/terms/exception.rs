use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Exception<Ty>
where
    Ty: Type,
{
    ty: Ty,
}

impl<Ty> Term for Exception<Ty> where Ty: Type {}

impl<Ty> fmt::Display for Exception<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}
