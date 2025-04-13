use super::{to_check_err, TypingEnv};
use crate::{
    syntax::{False, If, True},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for True {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingEnv;

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
    type Env = &'a mut TypingEnv;

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
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let ifc_ty = self.ifc.check(&mut env.clone())?;
        if let Type::Bool = ifc_ty {
            Ok(())
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: ifc_ty.to_string(),
                expected: "Bool".to_owned(),
            }))
        }?;

        let then_ty = self.thenc.check(&mut env.clone())?;
        let else_ty = self.elsec.check(env)?;
        if then_ty == else_ty {
            Ok(then_ty)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: then_ty.to_string(),
                expected: else_ty.to_string(),
            }))
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
