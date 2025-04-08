use super::Env;
use crate::{
    errors::{Error, ErrorKind},
    syntax::{
        terms::{IsZero, Pred, Succ, Zero},
        types::Type,
    },
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Zero {
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
        let inner = self.term.check(env)?;
        if inner != Type::Nat {
            return Err(Error::check(
                ErrorKind::TypeMismatch {
                    found: inner,
                    expected: "Nat".to_owned(),
                },
                self,
            ));
        }
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
        let inner = self.term.check(env)?;
        if inner != Type::Nat {
            return Err(Error::check(
                ErrorKind::TypeMismatch {
                    found: inner,
                    expected: "Nat".to_owned(),
                },
                self,
            ));
        }
        Ok(Type::Nat)
    }
}

impl<'a> Typecheck<'a> for IsZero {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner = self.term.check(env)?;
        if inner != Type::Nat {
            return Err(Error::check(
                ErrorKind::TypeMismatch {
                    found: inner,
                    expected: "Nat".to_owned(),
                },
                self,
            ));
        }
        Ok(Type::Nat)
    }
}
