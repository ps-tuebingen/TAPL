use super::{errors::Error, TypingContext};
use crate::{syntax::Fix, types::Type};
use common::Typecheck;

impl<'a> Typecheck<'a> for Fix {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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
