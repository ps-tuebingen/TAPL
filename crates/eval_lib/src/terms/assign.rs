use crate::{env::EvalEnvironment, Eval};
use common::errors::ValueMismatch;
use syntax::{
    terms::{Assign, Term},
    values::{Unit as UnitVal, ValueGroup},
};
use trace::EvalTrace;

impl<T> Eval for Assign<T>
where
    T: Term + Eval,
    UnitVal<T>: Into<<T as Eval>::Value>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let lhs_val = self.lhs.eval(env)?;
        let lhs_loc = lhs_val.into_loc()?;
        let rhs_val = self.rhs.eval(env)?;
        env.save_location(lhs_loc.loc, rhs_val);
        Ok(UnitVal::new().into())
    }
}
