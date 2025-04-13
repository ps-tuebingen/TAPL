use super::{to_check_err, TypingContext};
use crate::{
    syntax::{Pred, Succ, Zero},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Zero {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
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
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Nat)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                expected: Type::Nat.to_string(),
                found: inner_ty.to_string(),
            }))
        }
    }
}

impl<'a> Typecheck<'a> for Pred {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Nat)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                expected: Type::Nat.to_string(),
                found: inner_ty.to_string(),
            }))
        }
    }
}
