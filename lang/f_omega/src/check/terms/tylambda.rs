use super::{CheckType, Env};
use crate::{
    errors::Error,
    syntax::{
        terms::TyLambda,
        types::{Type, Universal},
    },
};

impl CheckType for TyLambda {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        env.add_tyvar(&self.var, &self.annot);
        let body_ty = self.body.check_type(env)?;
        Ok(Universal {
            var: self.var.clone(),
            kind: self.annot.clone(),
            ty: Box::new(body_ty),
        }
        .into())
    }
}
