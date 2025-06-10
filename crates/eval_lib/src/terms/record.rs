use crate::Eval;
use std::collections::HashMap;
use syntax::{
    terms::{Record, Term},
    values::Record as RecordVal,
    Label,
};
use trace::EvalTrace;

impl<T> Eval for Record<T>
where
    T: Term + Eval,
    RecordVal<<T as Eval>::Value>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let mut recs: HashMap<Label, Self::Value> = HashMap::new();
        for (lb, t) in self.records.into_iter() {
            let val = t.eval(env)?;
            recs.insert(lb, val);
        }
        Ok(RecordVal::<<T as Eval>::Value>::new::<Self::Value>(recs).into())
    }
}
