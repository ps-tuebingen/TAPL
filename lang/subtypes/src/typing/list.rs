use super::{errors::Error, meet, TypingContext};
use crate::{
    syntax::{Cons, ListCase, Nil},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Nil {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Err> {
        Ok(self.ty.clone())
    }
}

impl<'a> Typecheck<'a> for Cons {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fst_ty = self.fst.check(&mut env.clone())?;
        let rst_ty = self.rst.check(env)?;
        if let Type::List(ty) = rst_ty {
            let combined = meet(*ty, fst_ty);
            Ok(Type::List(Box::new(combined)))
        } else {
            Err(Error::NoList(rst_ty))
        }
    }
}

impl<'a> Typecheck<'a> for ListCase {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let nil_ty = self.nil_rhs.check(&mut env.clone())?;
        let list_inner = if let Type::List(inner) = self.list_ty.clone() {
            *inner
        } else {
            return Err(Error::NoList(self.list_ty.clone()));
        };
        env.add_var(&self.cons_fst, &list_inner);
        env.add_var(&self.cons_rst, &self.list_ty.clone());
        let cons_ty = self.cons_rhs.check(&mut env.clone())?;
        let combined = meet(nil_ty, cons_ty);
        Ok(combined)
    }
}
