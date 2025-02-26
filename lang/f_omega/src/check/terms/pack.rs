use super::{CheckType, Env};
use crate::{
    check::CheckKind,
    errors::Error,
    syntax::{kinds::Kind, terms::Pack, types::Type},
    traits::SubstTy,
};

impl CheckType for Pack {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let outer_kind = self.outer_ty.check_kind(&mut env.clone())?;
        outer_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::check(knd, self))?;
        let ex = self
            .outer_ty
            .clone()
            .as_existential()
            .map_err(|knd| Error::check(knd, self))?;
        let ex_subst = ex.ty.clone().subst_ty(&ex.ty_var, self.inner_ty.clone());
        let t_ty = self.term.check_type(env)?;
        t_ty.check_equal(&ex_subst)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(ex.into())
    }
}
