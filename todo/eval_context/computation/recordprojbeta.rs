use super::{ComputationRule, Eval, EvalContext, Value};
use crate::eval::to_eval_err;
use common::errors::{Error, ErrorKind};
use common::Label;

#[derive(Debug, PartialEq, Eq)]
pub struct RecordProjBeta {
    pub record: Value,
    pub label: Label,
}

impl Eval for RecordProjBeta {
    fn eval(self) -> Result<Value, Error> {
        if let Value::Record(recs) = self.record {
            recs.get(&self.label)
                .cloned()
                .ok_or(to_eval_err(ErrorKind::UndefinedLabel(self.label)))
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: self.record.to_string(),
                expected: "Record".to_owned(),
            }))
        }
    }
}

impl From<RecordProjBeta> for ComputationRule {
    fn from(proj: RecordProjBeta) -> ComputationRule {
        ComputationRule::RecordProjBeta(proj)
    }
}

impl From<RecordProjBeta> for EvalContext {
    fn from(proj: RecordProjBeta) -> EvalContext {
        EvalContext::Computation(proj.into())
    }
}

#[cfg(test)]
mod recordprojbeta_tests {
    use super::{Eval, RecordProjBeta};
    use crate::eval_context::Value;
    use std::collections::HashMap;

    #[test]
    fn eval_recordprojbeta() {
        let result = RecordProjBeta {
            label: "label".to_owned(),
            record: Value::Record(HashMap::from([
                ("label".to_owned(), Value::Zero),
                ("other".to_owned(), Value::True),
            ])),
        }
        .eval()
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
