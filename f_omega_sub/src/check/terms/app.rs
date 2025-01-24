use crate::{
    check::{check_subtype, Check, Env},
    errors::Error,
    eval::Eval,
    syntax::{terms::App, types::Type},
};

impl Check for App {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        let fun_ty = self.fun.check(&mut env.clone())?.eval(&mut env.clone())?;
        let fun = fun_ty.as_fun().map_err(|knd| Error::check(knd, self))?;
        let arg_ty = self.arg.check(&mut env.clone())?.eval(env)?;
        check_subtype(&arg_ty, &fun.from, env)?;
        Ok(*fun.to)
    }
}
