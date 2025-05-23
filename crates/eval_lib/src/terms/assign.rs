use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Assign, Term};

impl<T> Eval for Assign<T>
where
    T: Term + Eval,
    UnitVal<T>: Into<<T as Term>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let lhs_val = self.lhs.eval(env)?;
        let lhs_loc = lhs_val.into_loc().map_err(to_eval_err)?;
        let rhs_val = self.rhs.eval(env)?;
        env.save_location(lhs_loc.loc, rhs_val);
        Ok(UnitVal::new().into())
    }
}
