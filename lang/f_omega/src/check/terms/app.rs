use super::Env;
use crate::{
    errors::Error,
    syntax::{terms::App, types::Type},
};
use common::Typecheck;

impl<'a> Typecheck<'a> for App {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        let fun = fun_ty.as_fun().map_err(|knd| Error::check(knd, self))?;
        let arg_ty = self.arg.check(env)?;
        fun.from
            .check_equal(&arg_ty)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(*fun.to)
    }
}
