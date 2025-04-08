use crate::{
    check::Env,
    errors::Error,
    syntax::{terms::Unpack, types::Type},
};
use common::Eval;
use common::Typecheck;

impl<'a> Typecheck<'a> for Unpack {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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
