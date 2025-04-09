use super::{errors::Error, TypingContext};
use crate::{syntax::Cast, types::Type};
use common::Typecheck;

impl<'a> Typecheck<'a> for Cast {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        self.term.check(env)?;
        Ok(self.ty.clone())
    }
}
