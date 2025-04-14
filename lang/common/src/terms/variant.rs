use super::Term;
use crate::{types::Type, Label};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Variant<T, Ty>
where
    T: Term,
    Ty: Type,
{
    label: Label,
    term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Term for Variant<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> fmt::Display for Variant<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} = {}> as {}", self.label, self.term, self.ty)
    }
}
