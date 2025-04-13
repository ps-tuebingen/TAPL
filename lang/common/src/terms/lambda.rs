use super::Term;
use crate::{types::Type, Var};

#[derive(Clone, Debug)]
pub struct Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub var: Var,
    pub annot: Ty,
    pub body: Box<T>,
}

impl<T, Ty> Term for Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
