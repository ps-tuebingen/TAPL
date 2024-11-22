use super::{errors::Error, Check, TypingEnv};
use crate::{
    terms::syntax::{Proj, Tup},
    types::Type,
};

impl Check for Tup {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let mut tys = vec![];
        for term in self.terms.iter() {
            let ty = term.check_local(env)?;
            tys.push(ty);
        }
        Ok(Type::Tup(tys))
    }
}

impl Check for Proj {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let tup_ty = self.tup.check(env)?;
        if let Type::Tup(tys) = tup_ty {
            tys.get(self.ind)
                .ok_or(Error::ProjectionOutOfBounds {
                    proj_ty: Type::Tup(tys.clone()),
                    ind: self.ind,
                })
                .cloned()
        } else {
            Err(Error::UnexpectedType {
                ty: tup_ty.clone(),
                term: self.clone().into(),
            })
        }
    }
}
