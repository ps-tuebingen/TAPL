use super::Value;
use crate::{terms::Term, types::Type, Var};

pub struct Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    var: Var,
    annot: Ty,
    body: T,
}

impl<T, Ty> Value for Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
