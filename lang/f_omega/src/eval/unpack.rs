use super::{to_eval_err, Value};
use crate::{
    syntax::terms::Unpack,
    traits::{SubstTerm, SubstTy},
};
use common::{errors::Error, Eval};

impl Eval<'_> for Unpack {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.clone().eval(_env)?;
        let (inner, val, _) = bound_val.as_pack().map_err(to_eval_err)?;
        self.in_term
            .subst(&self.bound_var, val.into())
            .subst_ty(&self.ty_var, inner)
            .eval(_env)
    }
}
