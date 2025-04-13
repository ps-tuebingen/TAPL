use super::{to_check_err, Env};
use crate::{
    terms::{App, Lambda},
    types::Type,
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Lambda {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        env.add_var(self.var.clone(), self.annot.clone());
        let ret_ty = self.body.check(env)?;
        Ok(Type::fun(self.annot.clone(), ret_ty))
    }
}

impl<'a> Typecheck<'a> for App {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        let (from, to) = fun_ty.as_fun().map_err(to_check_err)?;
        let arg_ty = self.arg.check(env)?;
        from.check_equal(&arg_ty).map_err(to_check_err)?;
        Ok(to)
    }
}
