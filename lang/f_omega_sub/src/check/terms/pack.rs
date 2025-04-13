use super::to_check_err;
use crate::{
    check::{check_subtype, Env},
    syntax::{terms::Pack, types::Type},
    traits::SubstTy,
};
use common::{errors::Error, Eval, Typecheck};

impl<'a> Typecheck<'a> for Pack {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let outer_evaled = self.outer_ty.clone().eval(&mut env.clone())?;
        let ex = outer_evaled.as_existential().map_err(to_check_err)?;
        let sup_evaled = ex.sup_ty.clone().eval(&mut env.clone())?;
        check_subtype(&self.inner_ty, &sup_evaled, &mut env.clone())?;
        let t_ty = self.term.check(&mut env.clone())?.eval(&mut env.clone())?;
        let ty_subst = ex
            .ty
            .clone()
            .subst_ty(&ex.var, self.inner_ty.clone())
            .eval(env)?;
        check_subtype(&t_ty, &ty_subst, env)?;
        Ok(ex.into())
    }
}
