use crate::{Eval, errors::EvalError};
use std::collections::HashMap;
use syntax::{
    Label,
    eval_context::EvalContext,
    terms::{Record, Term},
    values::Record as RecordVal,
};
use trace::EvalTrace;

impl<T> Eval for Record<T>
where
    T: Term + Eval<Term = T>,
    Record<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
    RecordVal<<T as Eval>::Value>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let mut recs: HashMap<Label, Self::Value> = HashMap::new();
        let mut old_recs = self.records.clone();
        let mut steps = vec![];
        for (lb, t) in self.records.into_iter() {
            let res = t.eval(env)?;
            let val = res.val();
            recs.insert(lb.clone(), val.clone());

            let rule_recs = old_recs.clone();
            let lb_clone = lb.clone();
            steps.extend(
                res.congruence(&move |t| {
                    let mut recs_mut = rule_recs.clone();
                    recs_mut.insert(lb_clone.clone(), t);
                    Record::new(recs_mut).into()
                })
                .into_iter(),
            );
            old_recs.insert(lb, val.into());
        }
        let val = RecordVal::<<T as Eval>::Value>::new::<Self::Value>(recs);
        Ok(EvalTrace::new(steps, val))
    }
}
