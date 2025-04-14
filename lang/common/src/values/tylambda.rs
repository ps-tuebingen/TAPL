use super::Value;
use crate::{kinds::Kind, terms::Term, TypeVar};

pub struct TyLambda<T>
where
    T: Term,
{
    var: TypeVar,
    annot: Kind,
    term: T,
}

impl<T> Value for TyLambda<T> where T: Term {}
