use super::{errors::Error, is_subtype, Typecheck, TypingContext};
use crate::{syntax::Cast, types::Type};

impl Typecheck for Cast {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        let inner = self.term.check(env)?;
        if is_subtype(&inner, &self.ty) {
            Ok(self.ty.clone())
        } else {
            Err(Error::TypeMismatch(inner, self.ty.clone()))
        }
    }
}
