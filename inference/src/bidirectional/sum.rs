use super::{errors::Error, Environment, Infer};
use crate::{
    syntax::{Left, Right, SumCase},
    types::Type,
};

impl Infer for Left {
    fn infer(&self, _: &mut Environment) -> Result<Type, Error> {
        Err(Error::CannotInfer {
            t: self.clone().into(),
        })
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::Sum(ty1, _) = target {
            self.left_term.check(*ty1, env)
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}

impl Infer for Right {
    fn infer(&self, _: &mut Environment) -> Result<Type, Error> {
        Err(Error::CannotInfer {
            t: self.clone().into(),
        })
    }

    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::Sum(_, ty2) = target {
            self.right_term.check(*ty2, env)
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}

impl Infer for SumCase {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let bound_ty = self.bound_term.infer(env)?;
        if let Type::Sum(ty1, ty2) = bound_ty {
            let left_ty = self
                .left_term
                .infer_with(self.left_var.clone(), *ty1, env)?;
            let right_ty = self
                .right_term
                .infer_with(self.right_var.clone(), *ty2, env)?;
            if left_ty == right_ty {
                Ok(left_ty)
            } else {
                Err(Error::TypeMismatch {
                    ty1: left_ty,
                    ty2: right_ty,
                })
            }
        } else {
            Err(Error::TypeMismatch {
                ty1: bound_ty,
                ty2: Type::Sum(
                    Box::new("X".to_owned().into()),
                    Box::new("Y".to_owned().into()),
                ),
            })
        }
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        let bound_ty = self.bound_term.infer(env)?;
        if let Type::Sum(ty1, ty2) = bound_ty {
            self.left_term
                .check_with(self.left_var.clone(), *ty1, target.clone(), env)?;
            self.right_term
                .check_with(self.right_var.clone(), *ty2, target, env)
        } else {
            Err(Error::TypeMismatch {
                ty1: bound_ty,
                ty2: Type::Sum(
                    Box::new("X".to_owned().into()),
                    Box::new("Y".to_owned().into()),
                ),
            })
        }
    }
}

#[cfg(test)]
mod sum_tests {
    use super::{Infer, Left, Right, SumCase};
    use crate::{
        syntax::{Ascribe, IsZero, True, Zero},
        types::Type,
    };
    use std::collections::HashMap;

    #[test]
    fn infer_left() {
        let result = Left {
            left_term: Box::new(Zero.into()),
        }
        .infer(&mut HashMap::new());
        assert!(result.is_err())
    }

    #[test]
    fn check_left() {
        Left {
            left_term: Box::new(Zero.into()),
        }
        .check(
            Type::Sum(Box::new(Type::Nat), Box::new(Type::Bool)),
            &mut HashMap::new(),
        )
        .unwrap()
    }

    #[test]
    fn infer_right() {
        let result = Right {
            right_term: Box::new(True.into()),
        }
        .infer(&mut HashMap::new());
        assert!(result.is_err())
    }

    #[test]
    fn check_right() {
        Right {
            right_term: Box::new(True.into()),
        }
        .check(
            Type::Sum(Box::new(Type::Nat), Box::new(Type::Bool)),
            &mut HashMap::new(),
        )
        .unwrap()
    }

    #[test]
    fn infer_sumcase() {
        let result = SumCase {
            bound_term: Box::new(
                Ascribe {
                    term: Box::new(
                        Left {
                            left_term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                    ty: Type::Sum(Box::new(Type::Nat), Box::new(Type::Bool)),
                }
                .into(),
            ),
            left_var: "x".to_owned(),
            left_term: Box::new(
                IsZero {
                    term: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
            right_var: "y".to_owned(),
            right_term: Box::new("y".to_owned().into()),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_sumcase() {
        SumCase {
            bound_term: Box::new(
                Ascribe {
                    term: Box::new(
                        Left {
                            left_term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                    ty: Type::Sum(Box::new(Type::Nat), Box::new(Type::Bool)),
                }
                .into(),
            ),
            left_var: "x".to_owned(),
            left_term: Box::new(
                IsZero {
                    term: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
            right_var: "y".to_owned(),
            right_term: Box::new("y".to_owned().into()),
        }
        .check(Type::Bool, &mut HashMap::new())
        .unwrap()
    }
}
