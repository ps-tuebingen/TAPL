use super::{errors::Error, Check, Env};
use crate::{
    syntax::{App, Lambda},
    types::Type,
};

impl Check for Lambda {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        env.vars.insert(self.var, self.annot.clone());
        let ret_ty = self.body.check(env)?;
        Ok(Type::Fun(Box::new(self.annot), Box::new(ret_ty)))
    }
}

impl Check for App {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
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
