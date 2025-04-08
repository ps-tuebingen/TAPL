use super::Env;
use crate::{
    errors::Error,
    syntax::{
        terms::TyLambda,
        types::{Type, Universal},
    },
};
use common::Typecheck;

impl<'a> Typecheck<'a> for TyLambda {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        env.add_tyvar(&self.var, &self.annot);
        let body_ty = self.body.check(env)?;
        Ok(Universal {
            var: self.var.clone(),
            kind: self.annot.clone(),
            ty: Box::new(body_ty),
        }
        .into())
    }
}
