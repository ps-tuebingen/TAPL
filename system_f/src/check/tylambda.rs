use super::{errors::Error, Check, Env};
use crate::{
    syntax::{TyApp, TyLambda},
    types::Type,
};

impl Check for TyLambda {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        env.ty_vars.push(self.var.clone());
        let inner = self.term.check(env)?;
        Ok(Type::Forall(self.var, Box::new(inner)))
    }
}

impl Check for TyApp {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        let gen_ty = self.term.check(env)?;
        if let Type::Forall(v, ty) = gen_ty {
            Ok(ty.subst(&v, self.ty))
        } else {
            Err(Error::NotUniversal(gen_ty))
        }
    }
}
