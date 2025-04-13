use super::Type;
use crate::{kinds::Kind, TypeVar};

pub struct OpLambda<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    annot: Kind,
    body: Box<Ty>,
}

impl<Ty> Type for OpLambda<Ty> where Ty: Type {}
