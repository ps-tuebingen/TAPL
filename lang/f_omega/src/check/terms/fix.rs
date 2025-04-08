use super::Env;
use crate::{
    errors::Error,
    syntax::{terms::Fix, types::Type},
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Fix {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner_ty = self.term.check(env)?;
        let fun = inner_ty.as_fun().map_err(|knd| Error::check(knd, self))?;
        fun.from
            .check_equal(&fun.to)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(*fun.from)
    }
}
