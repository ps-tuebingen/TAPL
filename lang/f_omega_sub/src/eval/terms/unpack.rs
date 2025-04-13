use super::{to_eval_err, Env, Value};
use crate::{
    syntax::terms::Unpack,
    traits::{SubstTerm, SubstTy},
};
use common::{errors::Error, Eval};

impl<'a> Eval<'a> for Unpack {
    type Value = Value;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.clone().eval(&mut env.clone())?;
        let (inner, val, _) = bound_val.as_pack().map_err(to_eval_err)?;
        self.in_term
            .subst_ty(&self.ty_var, inner)
            .subst(&self.bound_var, val.into())
            .eval(env)
    }
}
