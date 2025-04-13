use super::{to_check_err, Env};
use crate::{terms::Fix, types::Type};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Fix {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let inner = self.term.check(env)?;
        let (from, to) = inner.as_fun().map_err(to_check_err)?;
        from.equal(&to).map_err(to_check_err)
    }
}
