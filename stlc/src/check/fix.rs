use super::{errors::Error, Check, TypingEnv};
use crate::{terms::syntax::Fix, types::Type};

impl Check for Fix {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let ty = self.term.check(env)?;
        if let Type::Fun(ty1, ty2) = ty {
            if *ty1 == *ty2 {
                Ok(*ty1)
            } else {
                Err(Error::TypeMismatch {
                    types: vec![*ty1, *ty2],
                })
            }
        } else {
            Err(Error::UnexpectedType {
                ty,
                term: self.clone().into(),
            })
        }
    }
}
