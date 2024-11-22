use super::{Check, TypingEnv};
use crate::{
    terms::syntax::{Proj, Tup},
    types::Type,
};

impl Check for Tup {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let mut tys = vec![];
        for term in self.terms.iter() {
            let ty = term.check_local(env)?;
            tys.push(ty);
        }
        Some(Type::Tup(tys))
    }
}

impl Check for Proj {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let tup_ty = self.tup.check(env)?;
        if let Type::Tup(tys) = tup_ty {
            tys.get(self.ind).cloned()
        } else {
            None
        }
    }
}
