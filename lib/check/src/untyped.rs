use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, kinds::Kind, terms::Term, untyped::Untyped};

impl<T> Subtypecheck for Untyped<T>
where
    T: Term,
{
    type Type = Untyped<T>;
    type Term = T;
    fn check_subtype(
        &self,
        _: &Self,
        _: Environment<Untyped<T>>,
    ) -> Result<Derivation<T, Untyped<T>>, CheckError> {
        Ok(SubtypeDerivation::empty().into())
    }
}

impl<T> Kindcheck<Untyped<T>> for Untyped<T>
where
    T: Term,
{
    fn check_kind(&self, _: Environment<Untyped<T>>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
    }
}

impl<T> Normalize<Untyped<T>> for Untyped<T>
where
    T: Term,
{
    fn normalize(self, _: Environment<Untyped<T>>) -> Self {
        self
    }
}
