use super::{AsContext, Error, EvalContext};
use crate::{
    eval_context::{computation::RecordProjBeta, congruence, Value},
    syntax::{Record, RecordProj},
};
use std::collections::HashMap;

impl AsContext for Record {
    fn to_context(self) -> Result<EvalContext, Error> {
        let mut vals = HashMap::new();
        let mut non_vals = HashMap::new();
        let mut last_label = "".to_owned();
        for (label, term) in self.records.into_iter() {
            match (&term).try_into() {
                Ok(val) => {
                    vals.insert(label, val);
                }
                Err(_) => {
                    last_label = label.clone();
                    non_vals.insert(label, term);
                }
            };
        }
        match non_vals.remove(&last_label) {
            Some(n) => Ok(congruence::Record {
                vals,
                label: last_label,
                next: Box::new(n.to_context()?),
                rest: non_vals,
            }
            .into()),

            None => Ok(EvalContext::Value(Value::Record(vals))),
        }
    }
}

impl AsContext for RecordProj {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.record).try_into() {
            Ok(val) => Ok(RecordProjBeta {
                record: val,
                label: self.label,
            }
            .into()),
            Err(_) => {
                let rec_ctx = (*self.record).to_context()?;
                Ok(congruence::RecordProj {
                    term: Box::new(rec_ctx),
                    label: self.label,
                }
                .into())
            }
        }
    }
}

#[cfg(test)]
mod record_tests {
    use super::{AsContext, EvalContext, Record, RecordProj};
    use crate::{
        eval_context::{
            computation::{IsZeroNum, RecordProjBeta},
            congruence, Value,
        },
        syntax::{IsZero, True, Zero},
    };
    use std::collections::HashMap;

    #[test]
    fn ctx_rec_cong() {
        let result = Record {
            records: HashMap::from([
                (
                    "label".to_owned(),
                    IsZero {
                        term: Box::new(Zero.into()),
                    }
                    .into(),
                ),
                ("other".to_owned(), Zero.into()),
            ]),
        }
        .to_context()
        .unwrap();
        let expected = congruence::Record {
            vals: HashMap::from([("other".to_owned(), Value::Zero)]),
            label: "label".to_owned(),
            next: Box::new(IsZeroNum { num: Value::Zero }.into()),
            rest: HashMap::new(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_rec_val() {
        let result = Record {
            records: HashMap::from([
                ("label".to_owned(), True.into()),
                ("other".to_owned(), Zero.into()),
            ]),
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Record(HashMap::from([
            ("label".to_owned(), Value::True),
            ("other".to_owned(), Value::Zero),
        ])));
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_proj_beta() {
        let result = RecordProj {
            label: "label".to_owned(),
            record: Box::new(
                Record {
                    records: HashMap::from([
                        ("label".to_owned(), True.into()),
                        ("other".to_owned(), Zero.into()),
                    ]),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = RecordProjBeta {
            label: "label".to_owned(),
            record: Value::Record(HashMap::from([
                ("label".to_owned(), Value::True),
                ("other".to_owned(), Value::Zero),
            ])),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_proj_cong() {
        let result = RecordProj {
            record: Box::new(
                Record {
                    records: HashMap::from([
                        (
                            "label".to_owned(),
                            IsZero {
                                term: Box::new(Zero.into()),
                            }
                            .into(),
                        ),
                        ("other".to_owned(), Zero.into()),
                    ]),
                }
                .into(),
            ),
            label: "label".to_owned(),
        }
        .to_context()
        .unwrap();
        let expected = congruence::RecordProj {
            label: "label".to_owned(),
            term: Box::new(
                congruence::Record {
                    vals: HashMap::from([("other".to_owned(), Value::Zero)]),
                    label: "label".to_owned(),
                    next: Box::new(IsZeroNum { num: Value::Zero }.into()),
                    rest: HashMap::new(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
