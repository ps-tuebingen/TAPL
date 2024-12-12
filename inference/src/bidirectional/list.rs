use super::{errors::Error, Environment, Infer};
use crate::{
    syntax::{Cons, Head, IsNil, Nil, Tail},
    types::Type,
};

impl Infer for Nil {
    fn infer(&self, _: &mut Environment) -> Result<Type, Error> {
        Err(Error::CannotInfer {
            t: self.clone().into(),
        })
    }
    fn check(&self, target: Type, _: &mut Environment) -> Result<(), Error> {
        if let Type::List(_) = target {
            Ok(())
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}
impl Infer for Cons {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let inner_ty = self.fst.infer_local(env)?;
        let list_ty = Type::List(Box::new(inner_ty));
        self.rst.check(list_ty.clone(), env)?;
        Ok(list_ty)
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::List(ty) = target {
            self.fst.check_local((*ty).clone(), env)?;
            self.rst.check(Type::List(ty), env)
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}

impl Infer for IsNil {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let inner_ty = self.list.infer(env)?;
        if let Type::List(_) = inner_ty {
            Ok(Type::Bool)
        } else {
            Err(Error::TypeMismatch {
                ty1: inner_ty,
                ty2: Type::List(Box::new("X".to_owned().into())),
            })
        }
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::Bool = target {
            let list_ty = self.list.infer(env)?;
            if let Type::List(_) = list_ty {
                Ok(())
            } else {
                Err(Error::TypeMismatch {
                    ty1: list_ty,
                    ty2: Type::List(Box::new("X".to_owned().into())),
                })
            }
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}

impl Infer for Head {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let inner_ty = self.list.infer(env)?;
        if let Type::List(ty) = inner_ty {
            Ok(*ty)
        } else {
            Err(Error::TypeMismatch {
                ty1: inner_ty,
                ty2: Type::List(Box::new("X".to_owned().into())),
            })
        }
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        self.list.check(Type::List(Box::new(target)), env)
    }
}

impl Infer for Tail {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let inner_ty = self.list.infer(env)?;
        if let Type::List(_) = inner_ty {
            Ok(inner_ty)
        } else {
            Err(Error::TypeMismatch {
                ty1: inner_ty,
                ty2: Type::List(Box::new("X".to_owned().into())),
            })
        }
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::List(_) = target {
            self.list.check(target, env)
        } else {
            Err(Error::BadTarget {
                t: self.clone().into(),
                ty: target,
            })
        }
    }
}

#[cfg(test)]
mod list_tests {
    use super::{Cons, Head, Infer, IsNil, Nil, Tail};
    use crate::{syntax::Zero, types::Type};
    use std::collections::HashMap;

    #[test]
    fn infer_nil() {
        let result = Nil.infer(&mut HashMap::new());
        assert!(result.is_err())
    }

    #[test]
    fn check_nil() {
        Nil.check(Type::List(Box::new(Type::Nat)), &mut HashMap::new())
            .unwrap()
    }

    #[test]
    fn infer_cons() {
        let result = Cons {
            fst: Box::new(Zero.into()),
            rst: Box::new(Nil.into()),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::List(Box::new(Type::Nat));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_cons() {
        Cons {
            fst: Box::new(Zero.into()),
            rst: Box::new(Nil.into()),
        }
        .check(Type::List(Box::new(Type::Nat)), &mut HashMap::new())
        .unwrap()
    }

    #[test]
    fn infer_isnil() {
        let result = IsNil {
            list: Box::new(
                Cons {
                    fst: Box::new(Zero.into()),
                    rst: Box::new(Nil.into()),
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
    fn check_isnil() {
        IsNil {
            list: Box::new(
                Cons {
                    fst: Box::new(Zero.into()),
                    rst: Box::new(Nil.into()),
                }
                .into(),
            ),
        }
        .check(Type::Bool, &mut HashMap::new())
        .unwrap()
    }

    #[test]
    fn infer_head() {
        let result = Head {
            list: Box::new(
                Cons {
                    fst: Box::new(Zero.into()),
                    rst: Box::new(Nil.into()),
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
    fn check_head() {
        Head {
            list: Box::new(
                Cons {
                    fst: Box::new(Zero.into()),
                    rst: Box::new(Nil.into()),
                }
                .into(),
            ),
        }
        .check(Type::Nat, &mut HashMap::new())
        .unwrap()
    }

    #[test]
    fn infer_tail() {
        let result = Tail {
            list: Box::new(
                Cons {
                    fst: Box::new(Zero.into()),
                    rst: Box::new(Nil.into()),
                }
                .into(),
            ),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::List(Box::new(Type::Nat));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_tail() {
        Tail {
            list: Box::new(
                Cons {
                    fst: Box::new(Zero.into()),
                    rst: Box::new(Nil.into()),
                }
                .into(),
            ),
        }
        .check(Type::List(Box::new(Type::Nat)), &mut HashMap::new())
        .unwrap()
    }
}
