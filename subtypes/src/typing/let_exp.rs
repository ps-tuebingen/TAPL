use super::{errors::Error, Typecheck, TypingContext};
use crate::{syntax::Let, types::Type};

impl Typecheck for Let {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        env.add_var(&self.var, &bound_ty);
        self.in_term.check(env)
    }
}
