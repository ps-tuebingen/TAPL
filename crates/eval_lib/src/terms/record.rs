use crate::{values::Record as RecordVal, Eval};
use common::errors::Error;
use std::collections::HashMap;
use syntax::{
    terms::{Record, Term},
    Label,
};

impl<T> Eval for Record<T>
where
    T: Term + Eval,
    RecordVal<T>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let mut recs: HashMap<Label, Self::Value> = HashMap::new();
        for (lb, t) in self.records.into_iter() {
            let val = t.eval(env)?;
            recs.insert(lb, val);
        }
        Ok(RecordVal::<T>::new::<Self::Value>(recs).into())
    }
}
