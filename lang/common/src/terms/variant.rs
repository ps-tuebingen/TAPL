use super::Term;
use crate::{subst::SubstType, types::Type, Label, TypeVar};
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

impl<T, Ty> SubstType<Ty> for Variant<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Variant {
            label: self.label,
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
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
