use super::{Check, TypingEnv};
use crate::{
    terms::syntax::{False, If, True},
    types::Type,
};

impl Check for True {
    fn check(&self, _: &mut TypingEnv) -> Option<Type> {
        Some(Type::Bool)
    }
}

impl Check for False {
    fn check(&self, _: &mut TypingEnv) -> Option<Type> {
        Some(Type::Bool)
    }
}

impl Check for If {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        if let Some(Type::Bool) = self.ifc.check_local(env) {
            Some(())
        } else {
            None
        }?;

        let then_ty = self.thenc.check_local(env)?;
        let else_ty = self.elsec.check(env)?;
        if then_ty == else_ty {
            Some(then_ty)
        } else {
            None
        }
    }
}
