use super::{to_infer_err, Environment, Infer};
use crate::{
    syntax::{False, If, True},
    types::Type,
};
use common::errors::{Error, ErrorKind};

impl Infer for True {
    fn infer(&self, _: &mut Environment) -> Result<Type, Error> {
        Ok(Type::Bool)
    }

    fn check(&self, target: Type, _: &mut Environment) -> Result<(), Error> {
        if target == Type::Bool {
            Ok(())
        } else {
            Err(to_infer_err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: target.to_string(),
            }))
        }
    }
}

impl Infer for False {
    fn infer(&self, _: &mut Environment) -> Result<Type, Error> {
        Ok(Type::Bool)
    }

    fn check(&self, target: Type, _: &mut Environment) -> Result<(), Error> {
        if target == Type::Bool {
            Ok(())
        } else {
            Err(to_infer_err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: target.to_string(),
            }))
        }
    }
}

impl Infer for If {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        self.ifc.check(Type::Bool, env)?;
        let then_ty = self.thenc.infer_local(env)?;
        let else_ty = self.elsec.infer_local(env)?;
        if then_ty == else_ty {
            Ok(then_ty)
        } else {
            Err(to_infer_err(ErrorKind::TypeMismatch {
                found: then_ty.to_string(),
                expected: else_ty.to_string(),
            }))
        }
    }

    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        self.ifc.check_local(Type::Bool, env)?;
        self.thenc.check_local(target.clone(), env)?;
        self.elsec.check_local(target, env)
    }
}

#[cfg(test)]
mod bool_tests {
    use super::{False, If, Infer, True};
    use crate::{syntax::Zero, types::Type};
    use std::collections::HashMap;

    #[test]
    fn infer_true() {
        let result = True.infer(&mut HashMap::new()).unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_true() {
        True.check(Type::Bool, &mut HashMap::new()).unwrap()
    }

    #[test]
    fn infer_false() {
        let result = False.infer(&mut HashMap::new()).unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_false() {
        False.check(Type::Bool, &mut HashMap::new()).unwrap()
    }

    #[test]
    fn infer_if() {
        let result = If {
            ifc: Box::new(True.into()),
            thenc: Box::new(Zero.into()),
            elsec: Box::new(Zero.into()),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_if() {
        If {
            ifc: Box::new(True.into()),
            thenc: Box::new(Zero.into()),
            elsec: Box::new(Zero.into()),
        }
        .check(Type::Nat, &mut HashMap::new())
        .unwrap()
    }
}
