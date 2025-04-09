use super::{errors::Error, Env};
use crate::{
    syntax::{App, Lambda},
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
        env.vars.insert(self.var.clone(), self.annot.clone());
        let ret_ty = self.body.check(env)?;
        Ok(Type::Fun(Box::new(self.annot.clone()), Box::new(ret_ty)))
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
        let arg_ty = self.arg.check(env)?;
        if let Type::Fun(from, to) = fun_ty {
            if arg_ty == *from {
                Ok(*to)
            } else {
                Err(Error::TypeMismatch(arg_ty, *from))
            }
        } else {
            Err(Error::NotAFunctionType(fun_ty))
        }
    }
}
