use super::{Check, TypingEnv};
use crate::{terms::syntax::Ascribe, types::Type};

impl Check for Ascribe {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let ty1 = self.term.check(env)?;
        if self.ty == ty1 {
            Some(ty1)
        } else {
            None
        }
    }
}
