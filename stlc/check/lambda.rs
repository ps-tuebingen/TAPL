use super::{Check, TypingEnv};
use crate::{
    terms::syntax::{App, Lambda},
    types::Type,
};

impl Check for Lambda {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        env.used_vars.insert(self.var.clone(), self.annot.clone());
        let ty = self.body.check(env)?;
        Some(Type::Fun(Box::new(self.annot.clone()), Box::new(ty)))
    }
}

impl Check for App {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let ty1 = self.fun.check_local(env)?;
        if let Type::Fun(ty11, ty12) = ty1 {
            let ty2 = self.arg.check(env)?;
            if ty2 == *ty11 {
                Some(*ty12)
            } else {
                None
            }
        } else {
            None
        }
    }
}
