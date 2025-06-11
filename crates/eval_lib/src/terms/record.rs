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
    T: Term + Eval<Term = T>,
    Record<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
    RecordVal<<T as Eval>::Value>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let mut recs: HashMap<Label, Self::Value> = HashMap::new();
        let mut old_recs = self.records.clone();
        let mut steps = vec![];
        for (lb, t) in self.records.into_iter() {
            let res = t.eval(env)?;
            let val = res.val();
            recs.insert(lb.clone(), val);

            let rule_recs = old_recs.clone();
            steps.extend(
                res.congruence(&move |t| {
                    rule_recs.insert(lb, val.into());
                    Record::new(rule_recs).into()
                })
                .into_iter(),
            );
            old_recs.insert(lb, val.into());
        }
        let val = RecordVal::<<T as Eval>::Value>::new::<Self::Value>(recs);
        Ok(EvalTrace::new(steps, val))
    }
}
