use crate::{Kindcheck, Normalize, Subtypecheck, errors::CheckError};
use syntax::{env::Environment, kinds::Kind, untyped::Untyped};

impl Subtypecheck<Untyped> for Untyped {
    fn check_subtype(&self, _: &Self, _: Environment<Untyped>) -> Result<(), CheckError> {
        Ok(())
    }
}

impl Kindcheck<Untyped> for Untyped {
    fn check_kind(&self, _: Environment<Untyped>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
    }
}

impl Normalize<Untyped> for Untyped {
    fn normalize(self, _: Environment<Untyped>) -> Self {
        self
    }
}
