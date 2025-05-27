use crate::{Kindcheck, Normalize, Subtypecheck};
use common::errors::Error;
use syntax::{kinds::Kind, untyped::Untyped};

impl Subtypecheck<Untyped> for Untyped {
    type Env = ();

    fn check_subtype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
}

impl Kindcheck<Untyped> for Untyped {
    type Env = ();
    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}

impl Normalize<Untyped> for Untyped {
    type Env = ();
    fn normalize(self, _: &mut Self::Env) -> Self {
        self
    }
}
