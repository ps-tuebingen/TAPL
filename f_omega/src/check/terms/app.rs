use super::{CheckType, Env};
use crate::{
    errors::Error,
    syntax::{terms::App, types::Type},
};

impl CheckType for App {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let fun_ty = self.fun.check_type(&mut env.clone())?;
        let fun = fun_ty.as_fun().map_err(|knd| Error::check(knd, self))?;
        let arg_ty = self.arg.check_type(env)?;
        fun.from
            .check_equal(&arg_ty)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(*fun.to)
    }
}
