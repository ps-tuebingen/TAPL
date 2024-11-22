use super::{Check, TypingEnv};
use crate::{
    terms::syntax::{Nothing, Something},
    types::Type,
};

impl Check for Nothing {
    fn check(&self, _: &mut TypingEnv) -> Option<Type> {
        Some(self.inner_type.clone())
    }
}

impl Check for Something {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let ty = self.term.check(env)?;
        Some(Type::Optional(Box::new(ty)))
    }
}
