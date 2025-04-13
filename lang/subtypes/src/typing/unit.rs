use super::TypingContext;
use crate::{syntax::Unit, types::Type};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Unit {
    type Type = Type;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, _: Self::Env) -> Result<Self::Type, Error> {
        Ok(Type::Unit)
    }
}
