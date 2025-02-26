use super::{errors::Error, Environment, Infer};
use crate::{
    syntax::{Pair, Proj1, Proj2},
    types::Type,
};

impl Infer for Pair {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let left_ty = self.fst.infer(env)?;
        let right_ty = self.snd.infer(env)?;
        Ok(Type::Prod(Box::new(left_ty), Box::new(right_ty)))
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::Prod(ty1, ty2) = target {
            self.fst.check_local(*ty1, env)?;
            self.snd.check(*ty2, env)
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}

impl Infer for Proj1 {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let pair_ty = self.pair.infer(env)?;
        if let Type::Prod(ty1, _) = pair_ty {
            Ok(*ty1)
        } else {
            Err(Error::TypeMismatch {
                ty1: pair_ty,
                ty2: Type::Prod(
                    Box::new("X".to_owned().into()),
                    Box::new("Y".to_owned().into()),
                ),
            })
        }
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        let pair_ty = self.pair.infer(env)?;
        if let Type::Prod(ty1, _) = pair_ty {
            if target == *ty1 {
                Ok(())
            } else {
                Err(Error::TypeMismatch {
                    ty1: target,
                    ty2: *ty1,
                })
            }
        } else {
            Err(Error::TypeMismatch {
                ty1: pair_ty,
                ty2: Type::Prod(
                    Box::new("X".to_owned().into()),
                    Box::new("Y".to_owned().into()),
                ),
            })
        }
    }
}

impl Infer for Proj2 {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let pair_ty = self.pair.infer(env)?;
        if let Type::Prod(_, ty2) = pair_ty {
            Ok(*ty2)
        } else {
            Err(Error::TypeMismatch {
                ty1: pair_ty,
                ty2: Type::Prod(
                    Box::new("X".to_owned().into()),
                    Box::new("Y".to_owned().into()),
                ),
            })
        }
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        let pair_ty = self.pair.infer(env)?;
        if let Type::Prod(_, ty2) = pair_ty {
            if target == *ty2 {
                Ok(())
            } else {
                Err(Error::TypeMismatch {
                    ty1: target,
                    ty2: *ty2,
                })
            }
        } else {
            Err(Error::TypeMismatch {
                ty1: pair_ty,
                ty2: Type::Prod(
                    Box::new("X".to_owned().into()),
                    Box::new("Y".to_owned().into()),
                ),
            })
        }
    }
}

#[cfg(test)]
mod pair_tests {
    use super::{Infer, Pair, Proj1, Proj2};
    use crate::{
        syntax::{True, Zero},
        types::Type,
    };
    use std::collections::HashMap;

    #[test]
    fn infer_pair() {
        let result = Pair {
            fst: Box::new(Zero.into()),
            snd: Box::new(True.into()),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Prod(Box::new(Type::Nat), Box::new(Type::Bool));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_pair() {
        Pair {
            fst: Box::new(Zero.into()),
            snd: Box::new(True.into()),
        }
        .check(
            Type::Prod(Box::new(Type::Nat), Box::new(Type::Bool)),
            &mut HashMap::new(),
        )
        .unwrap()
    }

    #[test]
    fn infer_proj1() {
        let result = Proj1 {
            pair: Box::new(
                Pair {
                    fst: Box::new(Zero.into()),
                    snd: Box::new(True.into()),
                }
                .into(),
            ),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_proj1() {
        Proj1 {
            pair: Box::new(
                Pair {
                    fst: Box::new(Zero.into()),
                    snd: Box::new(True.into()),
                }
                .into(),
            ),
        }
        .check(Type::Nat, &mut HashMap::new())
        .unwrap()
    }

    #[test]
    fn infer_proj2() {
        let result = Proj2 {
            pair: Box::new(
                Pair {
                    fst: Box::new(Zero.into()),
                    snd: Box::new(True.into()),
                }
                .into(),
            ),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_proj2() {
        Proj2 {
            pair: Box::new(
                Pair {
                    fst: Box::new(Zero.into()),
                    snd: Box::new(True.into()),
                }
                .into(),
            ),
        }
        .check(Type::Bool, &mut HashMap::new())
        .unwrap()
    }
}
