use super::TypingContext;
use crate::{syntax::Cast, types::Type};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Cast {
    type Type = Type;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        self.term.check(env)?;
        Ok(self.ty.clone())
    }
}
