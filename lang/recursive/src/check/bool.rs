use super::{to_check_err, Env};
use crate::{
    terms::{False, If, True},
    types::Type,
};

use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for True {
    type Type = Type;
    type Env = &'a Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, _: Self::Env) -> Result<Self::Type, Error> {
        Ok(Type::Bool)
    }
}

impl<'a> Typecheck<'a> for False {
    type Type = Type;
    type Env = &'a Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, _: Self::Env) -> Result<Self::Type, Error> {
        Ok(Type::Bool)
    }
}

impl<'a> Typecheck<'a> for If {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let ifty = self.ifc.check(&mut env.clone())?;
        ifty.equal(&Type::Bool).map_err(to_check_err)?;
        let then_ty = self.thenc.check(&mut env.clone())?;
        let else_ty = self.elsec.check(env)?;
        then_ty.equal(&else_ty).map_err(to_check_err)
    }
}
