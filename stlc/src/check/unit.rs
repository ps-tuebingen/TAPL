use super::{errors::Error, Check, TypingEnv};
use crate::{terms::syntax::Unit, types::Type};

impl Check for Unit {
    fn check(&self, _: &mut TypingEnv) -> Result<Type, Error> {
        Ok(Type::Unit)
    }
}
