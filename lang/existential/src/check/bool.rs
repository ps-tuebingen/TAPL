use super::{to_check_err, Env};
use crate::{
    terms::{False, If, True},
    types::Type,
};
use common::{errors::Error, Typecheck};

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
        let if_ty = self.ifc.check(&mut env.clone())?;
        if_ty.check_equal(&Type::Bool).map_err(to_check_err)?;
        let then_ty = self.thenc.check(&mut env.clone())?;
        let else_ty = self.elsec.check(env)?;
        then_ty.check_equal(&else_ty).map_err(to_check_err)?;
        Ok(then_ty)
    }
}
