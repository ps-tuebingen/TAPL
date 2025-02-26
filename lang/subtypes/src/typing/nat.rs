use super::{errors::Error, Typecheck, TypingContext};
use crate::{
    syntax::{Pred, Succ, Zero},
    types::Type,
};

impl Typecheck for Zero {
    fn check(&self, _: &mut TypingContext) -> Result<Type, Error> {
        Ok(Type::Nat)
    }
}

impl Typecheck for Succ {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Nat)
        } else {
            Err(Error::TypeMismatch(Type::Nat, inner_ty))
        }
    }
}

impl Typecheck for Pred {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Nat)
        } else {
            Err(Error::TypeMismatch(Type::Nat, inner_ty))
        }
    }
}
