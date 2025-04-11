use super::{to_check_err, Env};
use crate::{
    syntax::{kinds::Kind, terms::Pack, types::Type},
    traits::SubstTy,
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Pack {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let outer_kind = self.outer_ty.check(&mut env.clone())?;
        outer_kind.check_equal(&Kind::Star).map_err(to_check_err)?;
        let ex = self
            .outer_ty
            .clone()
            .as_existential()
            .map_err(to_check_err)?;
        let ex_subst = ex.ty.clone().subst_ty(&ex.ty_var, self.inner_ty.clone());
        let t_ty = self.term.check(env)?;
        t_ty.check_equal(&ex_subst).map_err(to_check_err)?;
        Ok(ex.into())
    }
}
