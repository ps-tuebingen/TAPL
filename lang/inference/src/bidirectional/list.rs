use super::{to_infer_err, Environment, Infer};
use crate::{
    syntax::{Cons, Head, IsNil, Nil, Tail},
    types::Type,
};
use common::errors::{Error, ErrorKind};

impl Infer for Nil {
    fn infer(&self, _: &mut Environment) -> Result<Type, Error> {
        Err(to_infer_err(ErrorKind::Infer {
            term: self.to_string(),
            reason: "Unknonwn List Type for Nil".to_owned(),
        }))
    }
    fn check(&self, target: Type, _: &mut Environment) -> Result<(), Error> {
        if let Type::List(_) = target {
            Ok(())
        } else {
            Err(to_infer_err(ErrorKind::TypeMismatch {
                expected: target.to_string(),
                found: self.to_string(),
            }))
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
            Err(to_infer_err(ErrorKind::TypeMismatch {
                expected: target.to_string(),
                found: self.to_string(),
            }))
        }
    }
}

impl Infer for IsNil {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let inner_ty = self.list.infer(env)?;
        if let Type::List(_) = inner_ty {
            Ok(Type::Bool)
        } else {
            Err(to_infer_err(ErrorKind::TypeMismatch {
                found: inner_ty.to_string(),
                expected: "List Type".to_owned(),
            }))
        }
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::Bool = target {
            let list_ty = self.list.infer(env)?;
            if let Type::List(_) = list_ty {
                Ok(())
            } else {
                Err(to_infer_err(ErrorKind::TypeMismatch {
                    found: list_ty.to_string(),
                    expected: "List Type".to_owned(),
                }))
            }
        } else {
            Err(to_infer_err(ErrorKind::TypeMismatch {
                expected: target.to_string(),
                found: self.to_string(),
            }))
        }
    }
}

impl Infer for Head {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let inner_ty = self.list.infer(env)?;
        if let Type::List(ty) = inner_ty {
            Ok(*ty)
        } else {
            Err(to_infer_err(ErrorKind::TypeMismatch {
                found: inner_ty.to_string(),
                expected: "List Type".to_owned(),
            }))
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
            Err(to_infer_err(ErrorKind::TypeMismatch {
                found: inner_ty.to_string(),
                expected: "List Type".to_owned(),
            }))
        }
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::List(_) = target {
            self.list.check(target, env)
        } else {
            Err(to_infer_err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: target.to_string(),
            }))
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
