use crate::{
    check::{Check, Env},
    errors::Error,
    syntax::{terms::Unpack, types::Type},
};
use common::Eval;

impl Check for Unpack {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .eval(&mut env.clone())?;
        let ex = bound_ty
            .as_existential()
            .map_err(|knd| Error::check(knd, self))?;
        env.add_tyvar(&self.ty_var, &ex.sup_ty)?;
        env.add_var(&self.bound_var, &ex.ty);
        self.in_term.check(env)
    }
}
