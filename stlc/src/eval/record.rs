use super::{errors::Error, Eval, Value};
use crate::syntax::{Record, RecordProj};
use std::collections::HashMap;

impl Eval for Record {
    fn eval(self) -> Result<Value, Error> {
        let mut vals = HashMap::new();
        for (label, term) in self.records.into_iter() {
            let val = term.eval()?;
            vals.insert(label, val);
        }
        Ok(Value::Record(vals))
    }
}

impl Eval for RecordProj {
    fn eval(self) -> Result<Value, Error> {
        match self.record.eval()? {
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
        .eval()
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
        .eval()
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
