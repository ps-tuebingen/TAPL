use super::{errors::Error, Environment, Infer};
use crate::{
    syntax::{IsZero, Pred, Succ, Zero},
    types::Type,
};

impl Infer for Zero {
    fn infer(&self, _: &mut Environment) -> Result<Type, Error> {
        Ok(Type::Nat)
    }
    fn check(&self, target: Type, _: &mut Environment) -> Result<(), Error> {
        if let Type::Nat = target {
            Ok(())
        } else {
            Err(Error::BadTarget {
                t: self.clone().into(),
                ty: target,
            })
        }
    }
}
impl Infer for Succ {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        self.term.check(Type::Nat, env)?;
        Ok(Type::Nat)
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::Nat = target {
            self.term.check(Type::Nat, env)
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}
impl Infer for Pred {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        self.term.check(Type::Nat, env)?;
        Ok(Type::Nat)
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::Nat = target {
            self.term.check(Type::Nat, env)
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}

impl Infer for IsZero {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        self.term.check(Type::Nat, env)?;
        Ok(Type::Bool)
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::Bool = target {
            self.term.check(Type::Nat, env)
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}

#[cfg(test)]
mod nat_tests {
    use super::{Infer, IsZero, Pred, Succ, Zero};
    use crate::types::Type;
    use std::collections::HashMap;

    #[test]
    fn infer_zero() {
        let result = Zero.infer(&mut HashMap::new()).unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_zero() {
        Zero.check(Type::Nat, &mut HashMap::new()).unwrap()
    }

    #[test]
    fn infer_succ() {
        let result = Succ {
            term: Box::new(Zero.into()),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_succ() {
        Succ {
            term: Box::new(Zero.into()),
        }
        .check(Type::Nat, &mut HashMap::new())
        .unwrap()
    }

    #[test]
    fn infer_pred() {
        let result = Pred {
            term: Box::new(Zero.into()),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_pred() {
        Pred {
            term: Box::new(Zero.into()),
        }
        .check(Type::Nat, &mut HashMap::new())
        .unwrap()
    }

    #[test]
    fn infer_iszero() {
        let result = IsZero {
            term: Box::new(Zero.into()),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_iszero() {
        IsZero {
            term: Box::new(Zero.into()),
        }
        .check(Type::Bool, &mut HashMap::new())
        .unwrap()
    }
}
