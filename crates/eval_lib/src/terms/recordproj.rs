use crate::Eval;
use common::errors::{UndefinedLabel, ValueMismatch};
use syntax::{
    terms::{RecordProj, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for RecordProj<T>
where
    T: Term + Eval<Term = T>,
    RecordProj<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
    <T as Eval>::EvalError: From<ValueMismatch> + From<UndefinedLabel>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let term_res = self.record.eval(env)?;
        let term_val = term_res.val();
        let rec_val = term_val.into_record()?;
        let val = rec_val
            .records
            .get(&self.label)
            .ok_or(UndefinedLabel::new(&self.label).into())
            .cloned()?;

        let mut steps = term_res.congruence(&move |t| RecordProj::new(t, &self.label).into());
        let last_step =
            EvalStep::recordproj(RecordProj::new(val.clone(), &self.label), val.clone());
        steps.push(last_step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
