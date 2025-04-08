use super::Value;
use crate::{
    errors::Error,
    syntax::terms::Unpack,
    traits::{SubstTerm, SubstTy},
};
use common::Eval;

impl Eval<'_> for Unpack {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let bound_val = self.bound_term.clone().eval(_env)?;
        let (inner, val, _) = bound_val.as_pack().map_err(|knd| Error::eval(knd, &self))?;
        self.in_term
            .subst(&self.bound_var, val.into())
            .subst_ty(&self.ty_var, inner)
            .eval(_env)
    }
}
