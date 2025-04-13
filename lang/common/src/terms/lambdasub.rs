use super::Term;
use crate::{types::Type, Var};

pub struct LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    var: Var,
    sup_ty: Ty,
    body: Box<T>,
}
