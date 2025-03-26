use super::{Check, Env, Error};
use crate::{
    syntax::{Const, Pred, Succ},
    types::Type,
};

impl Check for Const {
    fn check(&self, _: &mut Env) -> Result<Type, Error> {
        Ok(Type::Nat)
    }
}

impl Check for Succ {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let inner_ty = self.term.check(env)?;
        inner_ty
            .check_equal(&Type::Nat)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(Type::Nat)
    }
}

impl Check for Pred {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let inner_ty = self.term.check(env)?;
        inner_ty
            .check_equal(&Type::Nat)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(Type::Nat)
    }
}
