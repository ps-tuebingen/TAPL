use super::Env;
use crate::errors::{Error, ErrorKind};
use crate::syntax::{
    terms::{False, If, True},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for True {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Err> {
        Ok(Type::Bool)
    }
}

impl<'a> Typecheck<'a> for False {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Err> {
        Ok(Type::Bool)
    }
}

impl<'a> Typecheck<'a> for If {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let cond_ty = self.ifc.check(&mut env.clone())?;
        if cond_ty != Type::Bool {
            return Err(Error::check(
                ErrorKind::TypeMismatch {
                    found: cond_ty,
                    expected: "Bool".to_owned(),
                },
                self,
            ));
        }

        let then_ty = self.thent.check(&mut env.clone())?;
        let else_ty = self.elset.check(env)?;
        then_ty
            .check_equal(&else_ty)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(then_ty)
    }
}
