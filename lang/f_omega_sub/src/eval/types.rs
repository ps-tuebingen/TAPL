use super::{to_eval_err, Env};
use crate::{
    syntax::types::{Fun, RecordTy, Type},
    traits::SubstTy,
};
use common::{errors::Error, Eval};
use std::collections::HashMap;

impl<'a> Eval<'a> for Type {
    type Value = Self;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        match self {
            Type::Var(ref v) => env.get_tyvar(v).map_err(to_eval_err),
            Type::OpApp(app) => {
                let fun_evaled = app.fun.clone().eval(env)?;
                let lam = fun_evaled.as_oplambda().map_err(to_eval_err)?;
                lam.body.subst_ty(&lam.var, *app.arg).eval(env)
            }
            Type::Fun(fun) => {
                Ok(Fun::new(fun.from.eval(&mut env.clone())?, fun.to.eval(env)?).into())
            }
            Type::Universal(uni) => {
                env.add_tyvar(&uni.var, &uni.sup_ty)?;
                Ok(uni.into())
            }
            Type::Record(recs) => {
                let mut new_recs = HashMap::new();
                for (label, ty) in recs.records {
                    new_recs.insert(label, ty.eval(&mut env.clone())?);
                }
                Ok(RecordTy { records: new_recs }.into())
            }
            _ => Ok(self),
        }
    }
}
