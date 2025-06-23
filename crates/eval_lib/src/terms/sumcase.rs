use crate::{Eval, errors::EvalError};
use common::errors::{ValueKind, ValueMismatch};
use syntax::{
    eval_context::EvalContext,
    subst::SubstTerm,
    terms::{SumCase, Term},
    values::{Value, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for SumCase<T>
where
    T: Term + Eval<Term = T> + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    SumCase<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let bound_res = self.bound_term.eval(env)?;
        let bound_val = bound_res.val();

        let (res_steps, res_val) = if let Ok(left_val) = bound_val.clone().into_left() {
            let left_subst = self
                .left_term
                .clone()
                .subst(&self.left_var, &((*left_val.left_val).into()));
            let next_step = EvalStep::sumcase_left(
                SumCase::new(
                    bound_val,
                    &self.left_var,
                    *self.left_term.clone(),
                    &self.right_var,
                    *self.right_term.clone(),
                ),
                left_subst.clone(),
            );
            let left_res = left_subst.eval(env)?;
            let left_var = left_res.val();
            let mut left_steps = left_res.steps;
            left_steps.insert(0, next_step);
            (left_steps, left_var)
        } else if let Ok(right_val) = bound_val.clone().into_right() {
            let right_subst = self
                .right_term
                .clone()
                .subst(&self.right_var, &((*right_val.right_val).into()));
            let next_step = EvalStep::sumcase_right(
                SumCase::new(
                    bound_val,
                    &self.left_var,
                    *self.left_term.clone(),
                    &self.right_var,
                    *self.right_term.clone(),
                ),
                right_subst.clone(),
            );
            let right_res = right_subst.eval(env)?;
            let right_val = right_res.val();
            let mut right_steps = right_res.steps;
            right_steps.insert(0, next_step);
            (right_steps, right_val)
        } else {
            return Err(ValueMismatch::new(bound_val.knd(), ValueKind::Sum).into());
        };

        let mut steps = bound_res.congruence(&move |t| {
            SumCase::new(
                t,
                &self.left_var,
                *self.left_term.clone(),
                &self.right_var,
                *self.right_term.clone(),
            )
            .into()
        });
        steps.extend(res_steps);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, res_val))
    }
}
