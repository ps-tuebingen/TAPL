use super::{to_check_err, TypingEnv};
use crate::{
    syntax::{IsZero, Pred, Succ, Zero},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Zero {
    type Type = Type;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, _: Self::Env) -> Result<Self::Type, Error> {
        Ok(Type::Nat)
    }
}

impl<'a> Typecheck<'a> for Pred {
    type Type = Type;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let inner_ty = self.term.check(env)?;
        if let Type::Nat = inner_ty {
            Ok(Type::Nat)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: inner_ty.to_string(),
                expected: "Nat".to_owned(),
            }))
        }
    }
}

impl<'a> Typecheck<'a> for Succ {
    type Type = Type;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let inner_ty = self.term.check(env)?;
        if let Type::Nat = inner_ty {
            Ok(Type::Nat)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: inner_ty.to_string(),
                expected: "Nat".to_owned(),
            }))
        }
    }
}

impl<'a> Typecheck<'a> for IsZero {
    type Type = Type;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let inner_ty = self.term.check(env)?;
        if let Type::Nat = inner_ty {
            Ok(Type::Bool)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: inner_ty.to_string(),
                expected: "Nat".to_owned(),
            }))
        }
    }
}
#[cfg(test)]
mod nat_tests {
    use super::{Pred, Succ, Type, Zero};
    use common::Typecheck;

    #[test]
    fn check_zero() {
        let result = Zero.check(&mut Default::default()).unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_succ() {
        let result = Succ {
            term: Box::new(Zero.into()),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_pred() {
        let result = Pred {
            term: Box::new(Zero.into()),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }
}
