use super::{Env, Value};
use crate::{
    errors::Error,
    syntax::terms::Unpack,
    traits::{SubstTerm, SubstTy},
};
use common::Eval;

impl<'a> Eval<'a> for Unpack {
    type Value = Value;
    type Err = Error;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let bound_val = self.bound_term.clone().eval(&mut env.clone())?;
        let (inner, val, _) = bound_val
            .as_pack()
            .map_err(|knd| Error::eval(knd, self.clone()))?;
        self.in_term
            .subst_ty(&self.ty_var, inner)
            .subst(&self.bound_var, val.into())
            .eval(env)
    }
}
