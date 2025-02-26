use super::{Check, Env};
use crate::{errors::Error, terms::Let, types::Type};

impl Check for Let {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        env.insert(self.var.clone(), bound_ty);
        self.in_term.check(env)
    }
}
