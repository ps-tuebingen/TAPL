use super::{Check, TypingEnv};
use crate::{
    terms::syntax::{Cons, Head, IsNil, Nil, Tail},
    types::Type,
};

impl Check for Nil {
    fn check(&self, _: &mut TypingEnv) -> Option<Type> {
        Some(Type::List(Box::new(self.inner_type.clone())))
    }
}

impl Check for Cons {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let fst_ty = self.fst.check_local(env)?;
        if fst_ty == self.inner_type {
            Some(())
        } else {
            None
        }?;

        let rst_ty = self.rst.check(env)?;
        match rst_ty {
            Type::List(ty1) => {
                if self.inner_type == *ty1 {
                    Some(Type::List(ty1))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Check for IsNil {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let lst_ty = self.list.check(env)?;
        if let Type::List(ty1) = lst_ty {
            if *ty1 == self.inner_type {
                Some(Type::Bool)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Check for Head {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let lst_ty = self.list.check(env)?;
        if let Type::List(ty1) = lst_ty {
            if *ty1 == self.inner_type {
                Some(*ty1)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Check for Tail {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let lst_ty = self.list.check(env)?;
        if let Type::List(ty1) = lst_ty {
            if self.inner_type == *ty1 {
                Some(Type::List(ty1))
            } else {
                None
            }
        } else {
            None
        }
    }
}
