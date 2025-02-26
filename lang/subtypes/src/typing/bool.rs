use super::{errors::Error, meet, Typecheck, TypingContext};
use crate::{
    syntax::{False, If, True},
    types::Type,
};

impl Typecheck for True {
    fn check(&self, _: &mut TypingContext) -> Result<Type, Error> {
        Ok(Type::Bool)
    }
}

impl Typecheck for False {
    fn check(&self, _: &mut TypingContext) -> Result<Type, Error> {
        Ok(Type::Bool)
    }
}

impl Typecheck for If {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
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
