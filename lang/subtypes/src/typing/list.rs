use super::{meet, to_check_err, TypingContext};
use crate::{
    syntax::{Cons, ListCase, Nil},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Nil {
    type Type = Type;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, _: Self::Env) -> Result<Self::Type, Error> {
        Ok(self.ty.clone())
    }
}

impl<'a> Typecheck<'a> for Cons {
    type Type = Type;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let fst_ty = self.fst.check(&mut env.clone())?;
        let rst_ty = self.rst.check(env)?;
        if let Type::List(ty) = rst_ty {
            let combined = meet(*ty, fst_ty);
            Ok(Type::List(Box::new(combined)))
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: rst_ty.to_string(),
                expected: "List Type".to_owned(),
            }))
        }
    }
}

impl<'a> Typecheck<'a> for ListCase {
    type Type = Type;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let nil_ty = self.nil_rhs.check(&mut env.clone())?;
        let list_inner = if let Type::List(inner) = self.list_ty.clone() {
            *inner
        } else {
            return Err(to_check_err(ErrorKind::TypeMismatch {
                found: self.list_ty.to_string(),
                expected: "List Type".to_owned(),
            }));
        };
        env.add_var(&self.cons_fst, &list_inner);
        env.add_var(&self.cons_rst, &self.list_ty.clone());
        let cons_ty = self.cons_rhs.check(&mut env.clone())?;
        let combined = meet(nil_ty, cons_ty);
        Ok(combined)
    }
}
