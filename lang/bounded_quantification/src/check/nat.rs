use super::{to_check_err, Env, Error};
use crate::{
    syntax::{Const, Pred, Succ},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Const {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Err> {
        Ok(Type::Nat)
    }
}

impl<'a> Typecheck<'a> for Succ {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner_ty = self.term.check(env)?;
        inner_ty.check_equal(&Type::Nat).map_err(to_check_err)?;
        Ok(Type::Nat)
    }
}

impl<'a> Typecheck<'a> for Pred {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner_ty = self.term.check(env)?;
        inner_ty.check_equal(&Type::Nat).map_err(to_check_err)?;
        Ok(Type::Nat)
    }
}
