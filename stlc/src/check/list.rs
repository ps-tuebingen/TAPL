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
