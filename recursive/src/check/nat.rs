use super::{Check, Env};
use crate::{
    errors::Error,
    terms::{IsZero, Pred, Succ, Zero},
    types::Type,
};

impl Check for Zero {
    fn check(&self, _: &mut Env) -> Result<Type, Error> {
        Ok(Type::Nat)
    }
}

impl Check for Succ {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let inner = self.term.check(env)?;
        inner
            .equal(&Type::Nat)
            .map_err(|knd| Error::check(knd, self))
    }
}

impl Check for Pred {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let inner = self.term.check(env)?;
        inner
            .equal(&Type::Nat)
            .map_err(|knd| Error::check(knd, self))
    }
}

impl Check for IsZero {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let inner = self.term.check(env)?;
        inner
            .equal(&Type::Nat)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(Type::Bool)
    }
}
