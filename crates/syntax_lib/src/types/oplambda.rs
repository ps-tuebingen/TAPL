use super::{OpLambdaSub, Top, Type};
use crate::{kinds::Kind, subst::SubstType, TypeVar};
use common::errors::TypeKind;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpLambda<Ty>
where
    Ty: Type,
{
    pub var: TypeVar,
    pub annot: Kind,
    pub body: Box<Ty>,
}

impl<Ty> OpLambda<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(var: &str, knd: Kind, ty: Ty1) -> OpLambda<Ty>
    where
        Ty1: Into<Ty>,
    {
        OpLambda {
            var: var.to_owned(),
            annot: knd,
            body: Box::new(ty.into()),
        }
    }

    pub fn to_oplambda_unbounded(self) -> OpLambdaSub<Ty>
    where
        Top<Ty>: Into<Ty>,
    {
        OpLambdaSub::new_unbounded(&self.var, self.annot, *self.body)
    }
}

impl<Ty> Type for OpLambda<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::OpLambda
    }
}

impl<Ty> SubstType<Ty> for OpLambda<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.var {
            self.into()
        } else {
            OpLambda {
                var: self.var,
                annot: self.annot,
                body: Box::new(self.body.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<Ty> fmt::Display for OpLambda<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.{}", self.var, self.annot, self.body)
    }
}
