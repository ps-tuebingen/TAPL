use super::{errors::Error, Check, TypingEnv};
use crate::{
    syntax::{Pair, Proj1, Proj2},
    types::Type,
};

impl Check for Pair {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let ty1 = self.fst.check_local(env)?;
        let ty2 = self.snd.check(env)?;
        Ok(Type::Prod(Box::new(ty1), Box::new(ty2)))
    }
}

impl Check for Proj1 {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let ty = self.pair.check(env)?;
        if let Type::Prod(ty1, _) = ty {
            Ok(*ty1)
        } else {
            Err(Error::UnexpectedType {
                ty,
                term: self.clone().into(),
            })
        }
    }
}

impl Check for Proj2 {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let ty = self.pair.check(env)?;
        if let Type::Prod(_, ty2) = ty {
            Ok(*ty2)
        } else {
            Err(Error::UnexpectedType {
                ty,
                term: self.clone().into(),
            })
        }
    }
}

#[cfg(test)]
mod pair_tests {
    use super::{Check, Pair, Proj1, Proj2};
    use crate::{
        syntax::{True, Zero},
        types::Type,
    };

    #[test]
    fn check_pair() {
        let result = Pair {
            fst: Box::new(Zero.into()),
            snd: Box::new(True.into()),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Prod(Box::new(Type::Nat), Box::new(Type::Bool));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_proj1() {
        let result = Proj1 {
            pair: Box::new(
                Pair {
                    fst: Box::new(Zero.into()),
                    snd: Box::new(True.into()),
                }
                .into(),
            ),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_proj2() {
        let result = Proj2 {
            pair: Box::new(
                Pair {
                    fst: Box::new(Zero.into()),
                    snd: Box::new(True.into()),
                }
                .into(),
            ),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }
}
