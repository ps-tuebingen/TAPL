use super::Env;
use crate::{errors::Error, terms::Fix, types::Type};
use common::Typecheck;

impl<'a> Typecheck<'a> for Fix {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner = self.term.check(env)?;
        let (from, to) = inner.as_fun().map_err(|knd| Error::check(knd, self))?;
        from.equal(&to).map_err(|knd| Error::check(knd, self))
    }
}
