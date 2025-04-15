use super::Value;
use crate::{
    kinds::Kind,
    terms::{Term, TyLambda as TyLambdaT},
    TypeVar,
};

pub struct TyLambda<T>
where
    T: Term,
{
    var: TypeVar,
    annot: Kind,
    term: T,
}

impl<T> Value<T> for TyLambda<T>
where
    T: Term,
{
    type Term = TyLambdaT<T>;
}

impl<T> From<TyLambda<T>> for TyLambdaT<T>
where
    T: Term,
{
    fn from(tylam: TyLambda<T>) -> TyLambdaT<T> {
        TyLambdaT::new(&tylam.var, tylam.annot, tylam.term)
    }
}
