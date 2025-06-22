use crate::Eval;
use common::errors::{ValueKind, ValueMismatch};
use syntax::{
    store::Store,
    terms::{If, Term},
    values::{Value, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for If<T>
where
    T: Term + Eval<Term = T>,
    If<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let cond_res = self.if_cond.eval(env)?;
        let cond_val = cond_res.val();
        let (next_step, branch_res) = if cond_val.clone().into_true().is_ok() {
            (
                EvalStep::if_true(
                    If::new(cond_val, *self.then_term.clone(), *self.else_term.clone()),
                    *self.then_term.clone(),
                ),
                self.then_term.clone().eval(env)?,
            )
        } else if cond_val.clone().into_false().is_ok() {
            (
                EvalStep::if_false(
                    If::new(cond_val, *self.then_term.clone(), *self.else_term.clone()),
                    *self.else_term.clone(),
                ),
                self.else_term.clone().eval(env)?,
            )
        } else {
            return Err(ValueMismatch::new(cond_val.knd(), ValueKind::Bool).into());
        };
        let branch_val = branch_res.val();

        let mut steps = cond_res.congruence(&move |t| {
            If::new(t, *self.then_term.clone(), *self.else_term.clone()).into()
        });
        steps.push(next_step);
        steps.extend(branch_res.steps);

        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, branch_val))
    }
}
