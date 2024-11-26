use super::{errors::Error, Check, TypingEnv};
use crate::{
    syntax::{Nothing, Something},
    types::Type,
};

impl Check for Nothing {
    fn check(&self, _: &mut TypingEnv) -> Result<Type, Error> {
        Ok(self.inner_type.clone())
    }
}

impl Check for Something {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let ty = self.term.check(env)?;
        Ok(Type::Optional(Box::new(ty)))
    }
}
