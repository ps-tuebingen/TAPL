use super::{Check, Env};
use crate::{errors::Error, terms::Fix, types::Type};

impl Check for Fix {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let inner = self.term.check(env)?;
        let (from, to) = inner.as_fun().map_err(|knd| Error::check(knd, self))?;
        from.equal(&to).map_err(|knd| Error::check(knd, self))
    }
}
