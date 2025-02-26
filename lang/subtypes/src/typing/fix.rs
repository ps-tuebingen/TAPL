use super::{errors::Error, Typecheck, TypingContext};
use crate::{syntax::Fix, types::Type};

impl Typecheck for Fix {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        let inner = self.term.check(env)?;
        if let Type::Fun { from, to } = inner {
            if from == to {
                Ok(*from)
            } else {
                Err(Error::TypeMismatch(*from, *to))
            }
        } else {
            Err(Error::NoFunction(inner))
        }
    }
}
