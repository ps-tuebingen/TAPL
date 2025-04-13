use super::Term;
use crate::types::Type;

pub struct Nil<Ty>
where
    Ty: Type,
{
    ty: Ty,
}

impl<Ty> Term for Nil<Ty> where Ty: Type {}
