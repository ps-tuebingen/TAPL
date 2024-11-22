use super::{Check, TypingEnv};
use crate::{terms::syntax::Fix, types::Type};

impl Check for Fix {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let ty = self.term.check(env)?;
        if let Type::Fun(ty1, ty2) = ty {
            if ty1 == ty2 {
                Some(*ty1)
            } else {
                None
            }
        } else {
            None
        }
    }
}
