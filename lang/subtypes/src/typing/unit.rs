use super::TypingContext;
use crate::{syntax::Unit, types::Type};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Unit {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Err> {
        Ok(Type::Unit)
    }
}
