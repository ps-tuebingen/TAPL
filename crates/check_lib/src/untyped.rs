use crate::{Kindcheck, Normalize, Subtypecheck};
use common::errors::NotImplemented;
use syntax::{env::Environment, kinds::Kind, untyped::Untyped};

impl Subtypecheck<Untyped> for Untyped {
    type CheckError = NotImplemented;

    fn check_subtype(
        &self,
        _: &Self,
        _: &mut Environment<Untyped>,
    ) -> Result<(), Self::CheckError> {
        Ok(())
    }
}

impl Kindcheck<Untyped> for Untyped {
    type CheckError = NotImplemented;

    fn check_kind(&self, _: &mut Environment<Untyped>) -> Result<Kind, Self::CheckError> {
        Ok(Kind::Star)
    }
}

impl Normalize<Untyped> for Untyped {
    fn normalize(self, _: &mut Environment<Untyped>) -> Self {
        self
    }
}
