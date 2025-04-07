use super::{CheckType, Env};
use crate::errors::{Error, ErrorKind};
use crate::syntax::{
    terms::{False, If, True},
    types::Type,
};

impl CheckType for True {
    fn check_type(&self, _: &mut Env) -> Result<Type, Error> {
        Ok(Type::Bool)
    }
}

impl CheckType for False {
    fn check_type(&self, _: &mut Env) -> Result<Type, Error> {
        Ok(Type::Bool)
    }
}

impl CheckType for If {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let cond_ty = self.ifc.check_type(&mut env.clone())?;
        if cond_ty != Type::Bool {
            return Err(Error::check(
                ErrorKind::TypeMismatch {
                    found: cond_ty,
                    expected: "Bool".to_owned(),
                },
                self,
            ));
        }

        let then_ty = self.thent.check_type(&mut env.clone())?;
        let else_ty = self.elset.check_type(env)?;
        then_ty
            .check_equal(&else_ty)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(then_ty)
    }
}
