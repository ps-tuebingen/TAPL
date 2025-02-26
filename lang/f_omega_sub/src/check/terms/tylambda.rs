use crate::{
    check::{Check, Env},
    errors::Error,
    eval::Eval,
    syntax::{
        terms::TyLambda,
        types::{Type, Universal},
    },
};

impl Check for TyLambda {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        env.add_tyvar(&self.var, &self.sup_ty)?;
        let body_ty = self.body.check(&mut env.clone())?.eval(env)?;
        Ok(Universal::new(self.var.as_str(), self.sup_ty.clone(), body_ty).into())
    }
}
