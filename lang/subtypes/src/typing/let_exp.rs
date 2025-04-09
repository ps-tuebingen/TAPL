use super::{errors::Error, TypingContext};
use crate::{syntax::Let, types::Type};
use common::Typecheck;

impl<'a> Typecheck<'a> for Let {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        env.add_var(&self.var, &bound_ty);
        self.in_term.check(env)
    }
}
