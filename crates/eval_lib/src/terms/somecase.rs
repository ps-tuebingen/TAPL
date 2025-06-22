use crate::{errors::EvalError, Eval};
use common::errors::{ValueKind, ValueMismatch};
use syntax::{
    eval_context::EvalContext,
    subst::SubstTerm,
    terms::{SomeCase, Term},
    values::{Value, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for SomeCase<T>
where
    T: Term + Eval<Term = T> + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    <T as Eval>::Value: Into<T>,
    SomeCase<T>: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let bound_res = self.bound_term.eval(env)?;
        let bound_val = bound_res.val();

        let (res_steps, res_val) = if let Ok(some_val) = bound_val.clone().into_something() {
            let some_subst = self
                .some_term
                .clone()
                .subst(&self.some_var, &((*some_val.val).into()));
            let next_step = EvalStep::somecase_some(
                SomeCase::new(
                    bound_val,
                    *self.none_term.clone(),
                    &self.some_var,
                    *self.some_term.clone(),
                ),
                some_subst.clone(),
            );
            let some_res = some_subst.eval(env)?;
            let some_val = some_res.val();
            let mut some_steps = some_res.steps;
            some_steps.insert(0, next_step);
            (some_steps, some_val)
        } else if bound_val.clone().into_nothing().is_ok() {
            let next_step = EvalStep::somecase_none(
                SomeCase::new(
                    bound_val,
                    *self.none_term.clone(),
                    &self.some_var,
                    *self.some_term.clone(),
                ),
                *self.none_term.clone(),
            );
            let none_res = self.none_term.clone().eval(env)?;
            let none_val = none_res.val();
            let mut none_steps = none_res.steps;
            none_steps.insert(0, next_step);
            (none_steps, none_val)
        } else {
            return Err(ValueMismatch::new(bound_val.knd(), ValueKind::Option).into());
        };

        let mut steps = bound_res.congruence(&move |t| {
            SomeCase::new(
                t,
                *self.none_term.clone(),
                &self.some_var,
                *self.some_term.clone(),
            )
            .into()
        });
        steps.extend(res_steps);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, res_val))
    }
}
