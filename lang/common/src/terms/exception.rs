use super::Term;
use crate::types::Type;

pub struct Exception<Ty>
where
    Ty: Type,
{
    ty: Ty,
}

impl<Ty> Term for Exception<Ty> where Ty: Type {}
