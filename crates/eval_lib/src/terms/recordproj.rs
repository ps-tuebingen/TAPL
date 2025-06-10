use crate::Eval;
use common::errors::{UndefinedLabel, ValueMismatch};
use syntax::{
    terms::{RecordProj, Term},
    values::ValueGroup,
};
use trace::EvalTrace;

impl<T> Eval for RecordProj<T>
where
    T: Term + Eval,
    <T as Eval>::EvalError: From<ValueMismatch> + From<UndefinedLabel>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.record.eval(env)?;
        let rec_val = term_val.into_record()?;
        rec_val
            .records
            .get(&self.label)
            .ok_or(UndefinedLabel::new(&self.label).into())
            .cloned()
    }
}
