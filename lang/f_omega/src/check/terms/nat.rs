use super::{CheckType, Env};
use crate::{
    errors::{Error, ErrorKind},
    syntax::{
        terms::{IsZero, Pred, Succ, Zero},
        types::Type,
    },
};

impl CheckType for Zero {
    fn check_type(&self, __: &mut Env) -> Result<Type, Error> {
        Ok(Type::Nat)
    }
}

impl CheckType for Succ {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let inner = self.term.check_type(env)?;
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

impl CheckType for Pred {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let inner = self.term.check_type(env)?;
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

impl CheckType for IsZero {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let inner = self.term.check_type(env)?;
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
