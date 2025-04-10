use super::{errors::Error, is_subtype, TypingContext};
use crate::{
    syntax::{App, Lambda},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Lambda {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
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
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        let (from, to) = if let Type::Fun { from, to } = fun_ty {
            (*from, *to)
        } else {
            return Err(Error::NoFunction(fun_ty));
        };
        let arg_ty = self.arg.check(env)?;
        if is_subtype(&arg_ty, &from) {
            Ok(to)
        } else {
            Err(Error::TypeMismatch(arg_ty, from))
        }
    }
}
