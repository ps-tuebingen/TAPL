use super::{to_check_err, Env};
use crate::{
    terms::{IsZero, Pred, Succ, Zero},
    types::Type,
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Zero {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, _: Self::Env) -> Result<Self::Type, Error> {
        Ok(Type::Nat)
    }
}

impl<'a> Typecheck<'a> for Succ {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let inner = self.term.check(env)?;
        inner.equal(&Type::Nat).map_err(to_check_err)
    }
}

impl<'a> Typecheck<'a> for Pred {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let inner = self.term.check(env)?;
        inner.equal(&Type::Nat).map_err(to_check_err)
    }
}

impl<'a> Typecheck<'a> for IsZero {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let inner = self.term.check(env)?;
        inner.equal(&Type::Nat).map_err(to_check_err)?;
        Ok(Type::Bool)
    }
}
