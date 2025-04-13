use super::to_check_err;
use crate::{
    check::{check_subtype, Env},
    syntax::{terms::App, types::Type},
};
use common::{errors::Error, Eval, Typecheck};

impl<'a> Typecheck<'a> for App {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fun_ty = self.fun.check(&mut env.clone())?.eval(&mut env.clone())?;
        let fun = fun_ty.as_fun().map_err(to_check_err)?;
        let arg_ty = self.arg.check(&mut env.clone())?.eval(env)?;
        check_subtype(&arg_ty, &fun.from, env)?;
        Ok(*fun.to)
    }
}
