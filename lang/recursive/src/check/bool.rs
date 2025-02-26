use super::{Check, Env};
use crate::{
    errors::Error,
    terms::{False, If, True},
    types::Type,
};

impl Check for True {
    fn check(&self, _: &mut Env) -> Result<Type, Error> {
        Ok(Type::Bool)
    }
}

impl Check for False {
    fn check(&self, _: &mut Env) -> Result<Type, Error> {
        Ok(Type::Bool)
    }
}

impl Check for If {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let ifty = self.ifc.check(&mut env.clone())?;
        ifty.equal(&Type::Bool)
            .map_err(|knd| Error::check(knd, self))?;
        let then_ty = self.thenc.check(&mut env.clone())?;
        let else_ty = self.elsec.check(env)?;
        then_ty
            .equal(&else_ty)
            .map_err(|knd| Error::check(knd, self))
    }
}
