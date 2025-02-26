use super::{errors::Error, Environment, Infer};
use crate::{
    syntax::{Nothing, SomeCase, Something},
    types::Type,
};

impl Infer for Nothing {
    fn infer(&self, _: &mut Environment) -> Result<Type, Error> {
        Err(Error::CannotInfer {
            t: self.clone().into(),
        })
    }

    fn check(&self, target: Type, _: &mut Environment) -> Result<(), Error> {
        if let Type::Optional(_) = target {
            Ok(())
        } else {
            Err(Error::BadTarget {
                t: self.clone().into(),
                ty: target,
            })
        }
    }
}

impl Infer for Something {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let inner_ty = self.term.infer(env)?;
        Ok(Type::Optional(Box::new(inner_ty)))
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::Optional(ty) = target {
            self.term.check(*ty, env)
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}

impl Infer for SomeCase {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let bound_ty = self.bound_term.infer(env)?;
        if let Type::Optional(ty) = bound_ty {
            let none_ty = self.none_rhs.infer(env)?;
            let some_ty = self.some_rhs.infer_with(self.some_var.clone(), *ty, env)?;
            if none_ty == some_ty {
                Ok(none_ty)
            } else {
                Err(Error::TypeMismatch {
                    ty1: none_ty,
                    ty2: some_ty,
                })
            }
        } else {
            Err(Error::TypeMismatch {
                ty1: bound_ty,
                ty2: Type::Optional(Box::new("X".to_owned().into())),
            })
        }
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        let bound_ty = self.bound_term.infer(env)?;
        if let Type::Optional(ty) = bound_ty {
            self.some_rhs
                .check_with(self.some_var.clone(), (*ty).clone(), target.clone(), env)?;
            self.none_rhs.check(target, env)
        } else {
            Err(Error::TypeMismatch {
                ty1: bound_ty,
                ty2: Type::Optional(Box::new("X".to_owned().into())),
            })
        }
    }
}

#[cfg(test)]
mod optional_tests {
    use super::{Infer, Nothing, SomeCase, Something};
    use crate::{syntax::Zero, types::Type};
    use std::collections::HashMap;

    #[test]
    fn infer_nothing() {
        let result = Nothing.infer(&mut HashMap::new());
        assert!(result.is_err())
    }

    #[test]
    fn check_nothing() {
        Nothing
            .check(Type::Optional(Box::new(Type::Nat)), &mut HashMap::new())
            .unwrap()
    }

    #[test]
    fn infer_something() {
        let result = Something {
            term: Box::new(Zero.into()),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Optional(Box::new(Type::Nat));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_something() {
        Something {
            term: Box::new(Zero.into()),
        }
        .check(Type::Optional(Box::new(Type::Nat)), &mut HashMap::new())
        .unwrap()
    }

    #[test]
    fn infer_somecase() {
        let result = SomeCase {
            bound_term: Box::new(
                Something {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
            none_rhs: Box::new(Zero.into()),
            some_var: "x".to_owned(),
            some_rhs: Box::new("x".to_owned().into()),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_somecase() {
        SomeCase {
            bound_term: Box::new(
                Something {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
            none_rhs: Box::new(Zero.into()),
            some_var: "x".to_owned(),
            some_rhs: Box::new("x".to_owned().into()),
        }
        .check(Type::Nat, &mut HashMap::new())
        .unwrap()
    }
}
