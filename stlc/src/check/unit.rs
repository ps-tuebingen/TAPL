use super::{Check, TypingEnv};
use crate::{terms::syntax::Unit, types::Type};

impl Check for Unit {
    fn check(&self, _: &mut TypingEnv) -> Option<Type> {
        Some(Type::Unit)
    }
}
