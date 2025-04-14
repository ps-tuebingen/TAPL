use super::Type;
use crate::{kinds::Kind, subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpLambda<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    annot: Kind,
    body: Box<Ty>,
}

impl<Ty> Type for OpLambda<Ty> where Ty: Type {}

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
