use super::{CheckType, Env};
use crate::{
    errors::Error,
    syntax::{terms::Unpack, types::Type},
};

impl CheckType for Unpack {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let bound_ty = self.bound_term.check_type(&mut env.clone())?;
        let ex = bound_ty
            .as_existential()
            .map_err(|knd| Error::check(knd, self))?;
        env.add_tyvar(&ex.ty_var, &ex.kind);
        env.add_var(&self.bound_var, &ex.ty);
        self.in_term.check_type(env)
    }
}
