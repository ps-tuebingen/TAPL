use super::Type;
use crate::{subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpLambdaSub<Ty>
where
    Ty: Type,
{
    pub var: TypeVar,
    pub sup: Box<Ty>,
    pub body: Box<Ty>,
}

impl<Ty> OpLambdaSub<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1, Ty2>(var: &str, sup: Ty1, ty: Ty2) -> OpLambdaSub<Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        OpLambdaSub {
            var: var.to_owned(),
            sup: Box::new(sup.into()),
            body: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for OpLambdaSub<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for OpLambdaSub<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        let sup_subst = self.sup.subst_type(v, ty);
        if *v == self.var {
            OpLambdaSub {
                var: self.var,
                sup: Box::new(sup_subst),
                body: self.body,
            }
            .into()
        } else {
            OpLambdaSub {
                var: self.var,
                sup: Box::new(sup_subst),
                body: Box::new(self.body.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<Ty> fmt::Display for OpLambdaSub<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:{}.{}", self.var, self.sup, self.body)
    }
}
