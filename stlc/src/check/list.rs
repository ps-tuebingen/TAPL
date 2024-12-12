use super::{errors::Error, Check, TypingEnv};
use crate::{
    syntax::{Cons, Head, IsNil, Nil, Tail},
    types::Type,
};

impl Check for Nil {
    fn check(&self, _: &mut TypingEnv) -> Result<Type, Error> {
        Ok(Type::List(Box::new(self.inner_type.clone())))
    }
}

impl Check for Cons {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let fst_ty = self.fst.check_local(env)?;
        if fst_ty == self.inner_type {
            Ok(())
        } else {
            Err(Error::WrongAscription {
                found: fst_ty,
                expected: self.inner_type.clone(),
            })
        }?;

        let rst_ty = self.rst.check(env)?;
        match rst_ty {
            Type::List(ty1) => {
                if self.inner_type == *ty1 {
                    Ok(Type::List(ty1))
                } else {
                    Err(Error::WrongAscription {
                        found: *ty1,
                        expected: self.inner_type.clone(),
                    })
                }
            }
            _ => Err(Error::UnexpectedType {
                ty: rst_ty,
                term: (*self.rst).clone(),
            }),
        }
    }
}

impl Check for IsNil {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let lst_ty = self.list.check(env)?;
        if let Type::List(ty1) = lst_ty {
            if *ty1 == self.inner_type {
                Ok(Type::Bool)
            } else {
                Err(Error::WrongAscription {
                    found: *ty1.clone(),
                    expected: self.inner_type.clone(),
                })
            }
        } else {
            Err(Error::UnexpectedType {
                ty: lst_ty.clone(),
                term: self.clone().into(),
            })
        }
    }
}

impl Check for Head {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let lst_ty = self.list.check(env)?;
        if let Type::List(ty1) = lst_ty {
            if *ty1 == self.inner_type {
                Ok(*ty1)
            } else {
                Err(Error::WrongAscription {
                    found: *ty1.clone(),
                    expected: self.inner_type.clone(),
                })
            }
        } else {
            Err(Error::UnexpectedType {
                ty: lst_ty.clone(),
                term: self.clone().into(),
            })
        }
    }
}

impl Check for Tail {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let lst_ty = self.list.check(env)?;
        if let Type::List(ty1) = lst_ty {
            if self.inner_type == *ty1 {
                Ok(Type::List(ty1))
            } else {
                Err(Error::WrongAscription {
                    found: *ty1.clone(),
                    expected: self.inner_type.clone(),
                })
            }
        } else {
            Err(Error::UnexpectedType {
                ty: lst_ty.clone(),
                term: self.clone().into(),
            })
        }
    }
}

#[cfg(test)]
mod list_tests {
    use super::{Check, Cons, Head, IsNil, Nil, Tail};
    use crate::{syntax::Zero, types::Type};

    #[test]
    fn check_nil() {
        let result = Nil {
            inner_type: Type::Bool,
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::List(Box::new(Type::Bool));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_cons() {
        let result = Cons {
            fst: Box::new(Zero.into()),
            inner_type: Type::Nat,
            rst: Box::new(
                Nil {
                    inner_type: Type::Nat,
                }
                .into(),
            ),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::List(Box::new(Type::Nat));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_isnil() {
        let result = IsNil {
            inner_type: Type::Nat,
            list: Box::new(
                Nil {
                    inner_type: Type::Nat,
                }
                .into(),
            ),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_head() {
        let result = Head {
            inner_type: Type::Nat,
            list: Box::new(
                Nil {
                    inner_type: Type::Nat,
                }
                .into(),
            ),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_tail() {
        let result = Tail {
            inner_type: Type::Bool,
            list: Box::new(
                Nil {
                    inner_type: Type::Bool,
                }
                .into(),
            ),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::List(Box::new(Type::Bool));
        assert_eq!(result, expected)
    }
}
