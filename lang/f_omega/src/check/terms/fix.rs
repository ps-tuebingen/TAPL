use super::{to_check_err, Env};
use crate::syntax::{terms::Fix, types::Type};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Fix {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner_ty = self.term.check(env)?;
        let fun = inner_ty.as_fun().map_err(to_check_err)?;
        fun.from.check_equal(&fun.to).map_err(to_check_err)?;
        Ok(*fun.from)
    }
}
