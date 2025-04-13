use super::{to_check_err, Env};
use crate::syntax::{terms::Unpack, types::Type};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Unpack {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        let ex = bound_ty.as_existential().map_err(to_check_err)?;
        env.add_tyvar(&ex.ty_var, &ex.kind);
        env.add_var(&self.bound_var, &ex.ty);
        self.in_term.check(env)
    }
}
