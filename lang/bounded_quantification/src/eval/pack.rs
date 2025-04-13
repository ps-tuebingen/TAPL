use super::{to_eval_err, Value};
use crate::{
    syntax::{Pack, Unpack},
    traits::{SubstTerm, SubstTy},
};
use common::{errors::Error, Eval};

impl Eval<'_> for Pack {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let t_evaled = self.term.eval(_env)?;
        Ok(Value::Pack {
            inner_ty: self.inner_ty,
            val: Box::new(t_evaled),
            outer_ty: self.outer_ty,
        })
    }
}

impl Eval<'_> for Unpack {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let bound_evaled = self.bound_term.clone().eval(_env)?;
        let (inner_ty, val, _) = bound_evaled.as_pack().map_err(to_eval_err)?;
        self.in_term
            .subst(&self.bound_var, val.into())
            .subst_ty(&self.ty_var, inner_ty)
            .eval(_env)
    }
}
