use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar, Var};
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

impl<T, Ty> SubstType<Ty> for Lambda<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Lambda {
            var: self.var,
            annot: self.annot.subst_type(v, ty),
            body: Box::new(self.body.subst_type(v, ty)),
        }
        .into()
    }
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
