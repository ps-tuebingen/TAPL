use super::{errors::Error, Check, TypingEnv};
use crate::{
    terms::syntax::{App, Lambda},
    types::Type,
};

impl Check for Lambda {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        env.used_vars.insert(self.var.clone(), self.annot.clone());
        let ty = self.body.check(env)?;
        Ok(Type::Fun(Box::new(self.annot.clone()), Box::new(ty)))
    }
}

impl Check for App {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let ty1 = self.fun.check_local(env)?;
        if let Type::Fun(ty11, ty12) = ty1 {
            let ty2 = self.arg.check(env)?;
            if ty2 == *ty11 {
                Ok(*ty12)
            } else {
                Err(Error::TypeMismatch {
                    types: vec![*ty11, ty2],
                })
            }
        } else {
            Err(Error::UnexpectedType {
                ty: ty1,
                term: self.clone().into(),
            })
        }
    }
}
