use crate::{Eval, errors::EvalError};
use common::errors::UndefinedLabel;
use syntax::{
    eval_context::EvalContext,
    terms::{RecordProj, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for RecordProj<T>
where
    T: Term + Eval<Term = T>,
    RecordProj<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.record.eval(env)?;
        let term_val = term_res.val();
        let rec_val = term_val.into_record()?;
        let val = rec_val
            .records
            .get(&self.label)
            .cloned()
            .ok_or(UndefinedLabel::new(&self.label))?;

        let last_step =
            EvalStep::recordproj(RecordProj::new(val.clone(), &self.label), val.clone());

        let mut steps = term_res.congruence(&move |t| RecordProj::new(t, &self.label).into());
        steps.push(last_step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
