use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use common::errors::Error;
use syntax::{kinds::Kind, terms::Term, untyped::Untyped};

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

impl<T> Typecheck for T
where
    T: Term,
{
    type Type = Untyped;
    type Env = ();

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(Untyped)
    }
}

impl Normalize<Untyped> for Untyped {
    type Env = ();
    fn normalize(self, _: &mut Self::Env) -> Self {
        self
    }
}
