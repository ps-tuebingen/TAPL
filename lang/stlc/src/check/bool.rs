use super::{errors::Error, TypingEnv};
use crate::{
    syntax::{False, If, True},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for True {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Error> {
        Ok(Type::Bool)
    }
}

impl<'a> Typecheck<'a> for False {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Error> {
        Ok(Type::Bool)
    }
}

impl<'a> Typecheck<'a> for If {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
        let ifc_ty = self.ifc.check(&mut env.clone())?;
        if let Type::Bool = ifc_ty {
            Ok(())
        } else {
            Err(Error::UnexpectedType {
                ty: ifc_ty,
                term: (*self.ifc).clone(),
            })
        }?;

        let then_ty = self.thenc.check(&mut env.clone())?;
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
    use super::{False, If, True};
    use crate::types::Type;
    use common::Typecheck;

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
