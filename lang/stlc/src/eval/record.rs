use super::{errors::Error, Value};
use crate::syntax::{Record, RecordProj};
use common::Eval;
use std::collections::HashMap;

impl Eval<'_> for Record {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let mut vals = HashMap::new();
        for (label, term) in self.records.into_iter() {
            let val = term.eval(env)?;
            vals.insert(label, val);
        }
        Ok(Value::Record(vals))
    }
}

impl Eval<'_> for RecordProj {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        match self.record.eval(env)? {
            Value::Record(records) => {
                records
                    .get(&self.label)
                    .cloned()
                    .ok_or(Error::UndefinedLabel {
                        label: self.label.clone(),
                    })
            }
            val => Err(Error::BadValue { val }),
        }
    }
}

#[cfg(test)]
mod record_tests {
    use super::{Eval, Record, RecordProj, Value};
    use crate::syntax::{True, Zero};
    use std::collections::HashMap;

    #[test]
    fn eval_rec() {
        let result = Record {
            records: HashMap::from([
                ("label1".to_owned(), Zero.into()),
                ("label2".to_owned(), True.into()),
            ]),
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::Record(HashMap::from([
            ("label1".to_owned(), Value::Zero),
            ("label2".to_owned(), Value::True),
        ]));
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_proj() {
        let result = RecordProj {
            record: Box::new(
                Record {
                    records: HashMap::from([
                        ("label1".to_owned(), Zero.into()),
                        ("label2".to_owned(), True.into()),
                    ]),
                }
                .into(),
            ),
            label: "label1".to_owned(),
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
