use super::{errors::Error, meet, Typecheck, TypingContext};
use crate::{
    syntax::{Cons, ListCase, Nil},
    types::Type,
};

impl Typecheck for Nil {
    fn check(&self, _: &mut TypingContext) -> Result<Type, Error> {
        Ok(self.ty.clone())
    }
}

impl Typecheck for Cons {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
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

impl Typecheck for ListCase {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
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
