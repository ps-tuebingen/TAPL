use super::{errors::Error, Check, TypingEnv};
use crate::{
    syntax::{False, If, True},
    types::Type,
};

impl Check for True {
    fn check(&self, _: &mut TypingEnv) -> Result<Type, Error> {
        Ok(Type::Bool)
    }
}

impl Check for False {
    fn check(&self, _: &mut TypingEnv) -> Result<Type, Error> {
        Ok(Type::Bool)
    }
}

impl Check for If {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let ifc_ty = self.ifc.check_local(env)?;
        if let Type::Bool = ifc_ty {
            Ok(())
        } else {
            Err(Error::UnexpectedType {
                ty: ifc_ty,
                term: (*self.ifc).clone(),
            })
        }?;

        let then_ty = self.thenc.check_local(env)?;
        let else_ty = self.elsec.check(env)?;
        if then_ty == else_ty {
            Ok(then_ty)
        } else {
            Err(Error::TypeMismatch {
                types: vec![then_ty, else_ty],
            })
        }
    }
}
