use super::{errors::Error, Typecheck, TypingContext};
use crate::{syntax::Cast, types::Type};

impl Typecheck for Cast {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        self.term.check(env)?;
        Ok(self.ty.clone())
    }
}
