use crate::{
    check::Env,
    syntax::{
        terms::TyLambda,
        types::{Type, Universal},
    },
};
use common::{errors::Error, Eval, Typecheck};

impl<'a> Typecheck<'a> for TyLambda {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        env.add_tyvar(&self.var, &self.sup_ty)?;
        let body_ty = self.body.check(&mut env.clone())?.eval(env)?;
        Ok(Universal::new(self.var.as_str(), self.sup_ty.clone(), body_ty).into())
    }
}
