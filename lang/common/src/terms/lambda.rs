use super::Term;
use crate::{types::Type, Var};
use std::fmt;

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

impl<T, Ty> fmt::Display for Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}:{}.{}", self.var, self.annot, self.body)
    }
}
