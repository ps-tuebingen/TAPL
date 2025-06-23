use crate::{Eval, errors::EvalError};
use common::errors::{ValueKind, ValueMismatch};
use syntax::{
    eval_context::EvalContext,
    terms::{False as FalseT, IsNil, Term, True as TrueT},
    types::Type,
    values::{False, True, Value, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T, Ty> Eval for IsNil<T, Ty>
where
    T: Term + Eval<Term = T>,
    Ty: Type,
    IsNil<T, Ty>: Into<T>,
    True<T>: Into<<T as Eval>::Value>,
    TrueT<T>: Into<T>,
    False<T>: Into<<T as Eval>::Value>,
    FalseT<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let (step, val) = if term_val.clone().into_nil().is_ok() {
            let last_step = EvalStep::isnil_true(self.ty.clone());
            (last_step, True::new().into())
        } else if term_val.clone().into_cons().is_ok() {
            let last_step = EvalStep::isnil_false(self.ty.clone());
            (last_step, False::new().into())
        } else {
            return Err(ValueMismatch::new(term_val.knd(), ValueKind::List).into());
        };
        let mut steps = term_res.congruence(&move |t| IsNil::new(t, self.ty.clone()).into());
        steps.push(step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
