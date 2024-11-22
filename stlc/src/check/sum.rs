use super::{errors::Error, Check, TypingEnv};
use crate::{
    terms::syntax::{Left, Right, SumCase},
    types::Type,
};

impl Check for Left {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let left_ty = self.left_term.check(env)?;
        Ok(Type::Sum(
            Box::new(left_ty),
            Box::new(self.right_ty.clone()),
        ))
    }
}

impl Check for Right {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let right_ty = self.right_term.check(env)?;
        Ok(Type::Sum(
            Box::new(self.left_ty.clone()),
            Box::new(right_ty),
        ))
    }
}

impl Check for SumCase {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let bound_ty = self.bound_term.check_local(env)?;
        if let Type::Sum(left_ty, right_ty) = bound_ty {
            env.used_vars.insert(self.left_var.clone(), *left_ty);
            let left_ty = self.left_term.check_local(env)?;
            env.used_vars.remove(&self.left_var);

            env.used_vars.insert(self.right_var.clone(), *right_ty);
            let right_ty = self.right_term.check(env)?;

            if left_ty == right_ty {
                Ok(left_ty)
            } else {
                Err(Error::TypeMismatch {
                    types: vec![left_ty, right_ty],
                })
            }
        } else {
            Err(Error::UnexpectedType {
                ty: bound_ty.clone(),
                term: self.clone().into(),
            })
        }
    }
}
