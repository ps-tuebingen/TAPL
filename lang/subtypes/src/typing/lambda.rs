use super::{errors::Error, is_subtype, Typecheck, TypingContext};
use crate::{
    syntax::{App, Lambda},
    types::Type,
};

impl Typecheck for Lambda {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        env.add_var(&self.var, &self.annot);
        let body_ty = self.body.check(env)?;
        Ok(Type::Fun {
            from: Box::new(self.annot.clone()),
            to: Box::new(body_ty),
        })
    }
}

impl Typecheck for App {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
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
