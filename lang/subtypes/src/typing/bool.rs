use super::{errors::Error, meet, TypingContext};
use crate::{
    syntax::{False, If, True},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for True {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
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
    type Env = &'a mut TypingContext;
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
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let ifc_ty = self.ifc.check(&mut env.clone())?;
        if ifc_ty != Type::Bool {
            return Err(Error::TypeMismatch(Type::Bool, ifc_ty));
        }

        let thenc_ty = self.thenc.check(&mut env.clone())?;
        let elsec_ty = self.elsec.check(env)?;
        let combined = meet(thenc_ty, elsec_ty);
        Ok(combined)
    }
}
