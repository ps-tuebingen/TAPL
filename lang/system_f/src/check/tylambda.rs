use super::{to_check_err, Env};
use crate::{
    syntax::{TyApp, TyLambda},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for TyLambda {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        env.ty_vars.push(self.var.clone());
        let inner = self.term.check(env)?;
        Ok(Type::Forall(self.var.clone(), Box::new(inner)))
    }
}

impl<'a> Typecheck<'a> for TyApp {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let gen_ty = self.term.check(env)?;
        if let Type::Forall(v, ty) = gen_ty {
            Ok(ty.subst(&v, self.ty.clone()))
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: gen_ty.to_string(),
                expected: "Generalied Type".to_owned(),
            }))
        }
    }
}
