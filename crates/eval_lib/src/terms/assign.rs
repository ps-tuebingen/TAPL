use crate::Eval;
use errors::eval_error::EvalError;

use syntax::{
    eval_context::EvalContext,
    terms::{Assign, Term, Unit},
    values::{Unit as UnitVal, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Assign<T>
where
    T: Term + Eval<Term = T>,
    UnitVal<T>: Into<<T as Eval>::Value>,
    <T as Eval>::Value: Into<T>,
    Assign<T>: Into<T>,
    Unit<T>: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let lhs_res = self.lhs.clone().eval(env)?;
        let lhs_val = lhs_res.val();
        let lhs_t: T = lhs_val.clone().into();
        let lhs_loc = lhs_val.into_loc()?;

        let rhs_res = self.rhs.clone().eval(env)?;
        let rhs_val = rhs_res.val();
        let rhs_t: T = rhs_val.clone().into();

        let mut steps = lhs_res.congruence(&move |t| Assign::new(t, *self.rhs.clone()).into());
        steps.extend(rhs_res.congruence(&move |t| Assign::new(*self.lhs.clone(), t).into()));
        env.save_location(lhs_loc.loc, rhs_val.clone());

        steps.push(EvalStep::assign(lhs_t, rhs_t));
        Ok(EvalTrace::new(steps, UnitVal::<T>::new()))
    }
}
