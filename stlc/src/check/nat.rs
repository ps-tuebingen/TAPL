use super::{errors::Error, Check, TypingEnv};
use crate::{
    syntax::{IsZero, Pred, Succ, Zero},
    types::Type,
};

impl Check for Zero {
    fn check(&self, _: &mut TypingEnv) -> Result<Type, Error> {
        Ok(Type::Nat)
    }
}

impl Check for Pred {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let inner_ty = self.term.check(env)?;
        if let Type::Nat = inner_ty {
            Ok(Type::Nat)
        } else {
            Err(Error::UnexpectedType {
                ty: inner_ty,
                term: self.clone().into(),
            })
        }
    }
}

impl Check for Succ {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let inner_ty = self.term.check(env)?;
        if let Type::Nat = inner_ty {
            Ok(Type::Nat)
        } else {
            Err(Error::UnexpectedType {
                ty: inner_ty,
                term: self.clone().into(),
            })
        }
    }
}

impl Check for IsZero {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let inner_ty = self.term.check(env)?;
        if let Type::Nat = inner_ty {
            Ok(Type::Bool)
        } else {
            Err(Error::UnexpectedType {
                ty: inner_ty,
                term: self.clone().into(),
            })
        }
    }
}
#[cfg(test)]
mod nat_tests {
    use super::{Check, Pred, Succ, Type, Zero};

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
