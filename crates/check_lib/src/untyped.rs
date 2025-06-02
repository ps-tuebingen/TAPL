use crate::{Kindcheck, Normalize, Subtypecheck};
use common::errors::NotImplemented;
use syntax::{kinds::Kind, untyped::Untyped};

impl Subtypecheck<Untyped> for Untyped {
    type Env = ();
    type CheckError = NotImplemented;

    fn check_subtype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Self::CheckError> {
        Ok(())
    }
}

impl Kindcheck<Untyped> for Untyped {
    type Env = ();
    type CheckError = NotImplemented;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Self::CheckError> {
        Ok(Kind::Star)
    }
}

impl Normalize<Untyped> for Untyped {
    type Env = ();
    fn normalize(self, _: &mut Self::Env) -> Self {
        self
    }
}
