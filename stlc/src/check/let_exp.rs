use super::{errors::Error, Check, TypingEnv};
use crate::{terms::syntax::Let, types::Type};

impl Check for Let {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let bound_ty = self.bound_term.check_local(env)?;
        env.used_vars.insert(self.var.clone(), bound_ty);
        self.in_term.check(env)
    }
}
