use crate::{
    check::{check_subtype, Check, Env},
    errors::Error,
    eval::Eval,
    syntax::{terms::Pack, types::Type},
    traits::SubstTy,
};

impl Check for Pack {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        let outer_evaled = self.outer_ty.clone().eval(&mut env.clone())?;
        let ex = outer_evaled
            .as_existential()
            .map_err(|err| Error::check(err, self))?;
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
