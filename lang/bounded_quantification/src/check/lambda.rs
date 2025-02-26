use super::{check_subtype, Check, Env};
use crate::{
    errors::Error,
    syntax::{App, Lambda},
    types::Type,
};

impl Check for Lambda {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        env.add_var(&self.var, &self.annot);
        let body_ty = self.body.check(env)?;
        Ok(Type::Fun {
            from: Box::new(self.annot.clone()),
            to: Box::new(body_ty),
        })
    }
}

impl Check for App {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        let (from, to) = fun_ty.as_fun().map_err(|knd| Error::check(knd, self))?;
        let arg_ty = self.arg.check(&mut env.clone())?;
        check_subtype(arg_ty, from, env)?;
        Ok(to)
    }
}
