use super::{Check, Env, Error, ErrorKind};
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
        if inner_ty == Type::Nat {
            Ok(Type::Nat)
        } else {
            Err(Error::check(
                ErrorKind::TypeMismatch {
                    found: inner_ty,
                    expected: "Nat".to_owned(),
                },
                self,
            ))
        }
    }
}

impl Check for Pred {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Nat)
        } else {
            Err(Error::check(
                ErrorKind::TypeMismatch {
                    found: inner_ty,
                    expected: "Nat".to_owned(),
                },
                self,
            ))
        }
    }
}
