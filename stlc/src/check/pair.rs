use super::{errors::Error, Check, TypingEnv};
use crate::{
    terms::syntax::{Pair, Proj1, Proj2},
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
