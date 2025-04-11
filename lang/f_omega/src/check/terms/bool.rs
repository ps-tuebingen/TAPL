use super::{to_check_err, Env};
use crate::syntax::{
    terms::{False, If, True},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for True {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Err> {
        Ok(Type::Bool)
    }
}

impl<'a> Typecheck<'a> for False {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Err> {
        Ok(Type::Bool)
    }
}

impl<'a> Typecheck<'a> for If {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let cond_ty = self.ifc.check(&mut env.clone())?;
        if cond_ty != Type::Bool {
            return Err(to_check_err(ErrorKind::TypeMismatch {
                found: cond_ty.to_string(),
                expected: "Bool".to_owned(),
            }));
        }

        let then_ty = self.thent.check(&mut env.clone())?;
        let else_ty = self.elset.check(env)?;
        then_ty.check_equal(&else_ty).map_err(to_check_err)?;
        Ok(then_ty)
    }
}
