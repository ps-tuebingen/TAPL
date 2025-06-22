use crate::{errors::EvalError, Eval};

use syntax::{
    store::Store,
    terms::{False as FalseT, IsZero, Term, True as TrueT},
    values::{False, True, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for IsZero<T>
where
    T: Term + Eval<Term = T>,
    IsZero<T>: Into<T>,
    True<T>: Into<<T as Eval>::Value>,
    TrueT<T>: Into<T>,
    False<T>: Into<<T as Eval>::Value>,
    FalseT<T>: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let inner_res = self.term.eval(env)?;
        let val = inner_res.val();
        let num = val.clone().into_num()?;
        let mut steps = inner_res.congruence(&move |t| IsZero::new(t).into());
        if num.num == 0 {
            steps.push(EvalStep::iszero_true(IsZero::new(val)));
            Ok(EvalTrace::new(steps, True::new()))
        } else {
            steps.push(EvalStep::iszero_false(IsZero::new(val)));
            Ok(EvalTrace::new(steps, False::new()))
        }
    }
}
