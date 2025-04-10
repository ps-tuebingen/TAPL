use super::{check_subtype, to_check_err, Env};
use crate::{
    syntax::{App, Lambda},
    types::Type,
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Lambda {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        env.add_var(&self.var, &self.annot);
        let body_ty = self.body.check(env)?;
        Ok(Type::Fun {
            from: Box::new(self.annot.clone()),
            to: Box::new(body_ty),
        })
    }
}

impl<'a> Typecheck<'a> for App {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        let (from, to) = fun_ty.as_fun().map_err(to_check_err)?;
        let arg_ty = self.arg.check(&mut env.clone())?;
        check_subtype(arg_ty, from, env)?;
        Ok(to)
    }
}
