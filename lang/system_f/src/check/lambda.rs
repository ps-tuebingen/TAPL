use super::{to_check_err, Env};
use crate::{
    syntax::{App, Lambda},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Lambda {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        env.vars.insert(self.var.clone(), self.annot.clone());
        let ret_ty = self.body.check(env)?;
        Ok(Type::Fun(Box::new(self.annot.clone()), Box::new(ret_ty)))
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
        let arg_ty = self.arg.check(env)?;
        if let Type::Fun(from, to) = fun_ty {
            if arg_ty == *from {
                Ok(*to)
            } else {
                Err(to_check_err(ErrorKind::TypeMismatch {
                    found: arg_ty.to_string(),
                    expected: from.to_string(),
                }))
            }
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: fun_ty.to_string(),
                expected: "Function Type".to_owned(),
            }))
        }
    }
}
