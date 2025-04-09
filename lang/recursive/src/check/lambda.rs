use super::Env;
use crate::{
    errors::Error,
    terms::{App, Lambda},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Lambda {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        env.insert(self.var.clone(), self.annot.clone());
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
        let (from, to) = fun_ty.as_fun().map_err(|knd| Error::check(knd, self))?;
        let arg_ty = self.arg.check(env)?;
        let _ = from.equal(&arg_ty).map_err(|knd| Error::check(knd, self))?;
        Ok(to)
    }
}
