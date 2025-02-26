use super::{errors::Error, Typecheck, TypingContext};
use crate::{syntax::Unit, types::Type};

impl Typecheck for Unit {
    fn check(&self, _: &mut TypingContext) -> Result<Type, Error> {
        Ok(Type::Unit)
    }
}
