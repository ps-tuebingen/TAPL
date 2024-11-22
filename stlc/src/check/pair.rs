use super::{Check, TypingEnv};
use crate::{
    terms::syntax::{Pair, Proj1, Proj2},
    types::Type,
};

impl Check for Pair {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let ty1 = self.fst.check_local(env)?;
        let ty2 = self.snd.check(env)?;
        Some(Type::Prod(Box::new(ty1), Box::new(ty2)))
    }
}

impl Check for Proj1 {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let ty = self.pair.check(env)?;
        if let Type::Prod(ty1, _) = ty {
            Some(*ty1)
        } else {
            None
        }
    }
}

impl Check for Proj2 {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let ty = self.pair.check(env)?;
        if let Type::Prod(_, ty2) = ty {
            Some(*ty2)
        } else {
            None
        }
    }
}
