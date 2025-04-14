use super::Value;
use crate::{terms::Term, types::Type, Var};

pub struct LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    var: Var,
    ty: Ty,
    t: Term,
}

impl<T, Ty> Value for LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
