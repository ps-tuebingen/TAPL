use super::Type;
use crate::{kinds::Kind, TypeVar};
use std::fmt;

#[derive(Clone, Debug)]
pub struct OpLambda<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    annot: Kind,
    body: Box<Ty>,
}

impl<Ty> Type for OpLambda<Ty> where Ty: Type {}

impl<Ty> fmt::Display for OpLambda<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.{}", self.var, self.annot, self.body)
    }
}
