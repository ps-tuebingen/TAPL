use crate::{
    check::{Check, Env},
    errors::Error,
    syntax::{terms::Let, types::Type},
};
use common::Eval;

impl Check for Let {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .eval(&mut env.clone())?;
        env.add_var(&self.var, &bound_ty);
        self.in_term.check(env)
    }
}
