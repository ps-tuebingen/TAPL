use super::Term;
use crate::types::Type;

pub struct Nothing<Ty> where Ty:Type{
    ty:Ty
}

impl<Ty> Term for Nothing<Ty> where Ty:Type{}
