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
        let t_ty = self.0.check(env)?;
        t_ty.check_equal(&Type::Nat)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(t_ty)
    }
}

impl Check for Pred {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let t_ty = self.0.check(env)?;
        t_ty.check_equal(&Type::Nat)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(t_ty)
    }
}

impl Check for IsZero {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let t_ty = self.0.check(env)?;
        t_ty.check_equal(&Type::Nat)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(Type::Bool)
    }
}
