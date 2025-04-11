use super::{to_check_err, Env};
use crate::syntax::{terms::App, types::Type};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for App {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        let fun = fun_ty.as_fun().map_err(to_check_err)?;
        let arg_ty = self.arg.check(env)?;
        fun.from.check_equal(&arg_ty).map_err(to_check_err)?;
        Ok(*fun.to)
    }
}
