use super::Env;
use crate::{
    errors::Error,
    syntax::types::{Fun, RecordTy, Type},
    traits::SubstTy,
};
use common::Eval;
use std::collections::HashMap;

impl<'a> Eval<'a> for Type {
    type Value = Self;
    type Error = Error;
    type Env = &'a mut Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Error> {
        match self {
            Type::Var(ref v) => env.get_tyvar(v).map_err(|knd| Error::ty_red(knd, self)),
            Type::OpApp(app) => {
                let fun_evaled = app.fun.clone().eval(env)?;
                let lam = fun_evaled
                    .as_oplambda()
                    .map_err(|knd| Error::ty_red(knd, app.clone()))?;
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
