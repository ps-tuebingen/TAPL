use super::{errors::Error, TypingEnv};
use crate::{
    syntax::{Pair, Proj1, Proj2},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Pair {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
        let ty1 = self.fst.check(&mut env.clone())?;
        let ty2 = self.snd.check(env)?;
        Ok(Type::Prod(Box::new(ty1), Box::new(ty2)))
    }
}

impl<'a> Typecheck<'a> for Proj1 {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
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

impl<'a> Typecheck<'a> for Proj2 {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
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
    use super::{Pair, Proj1, Proj2};
    use crate::{
        syntax::{True, Zero},
        types::Type,
    };
    use common::Typecheck;

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
