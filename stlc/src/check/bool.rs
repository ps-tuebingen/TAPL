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

#[cfg(test)]
mod bool_tests {
    use super::{Check, False, If, True};
    use crate::types::Type;

    #[test]
    fn check_true() {
        let result = True.check(&mut Default::default()).unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_false() {
        let result = False.check(&mut Default::default()).unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_if() {
        let result = If {
            ifc: Box::new(True.into()),
            thenc: Box::new(False.into()),
            elsec: Box::new(True.into()),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }
}
