use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::eval_context::computation::RecordProjBeta;
use common::Label;

#[derive(Debug, PartialEq, Eq)]
pub struct RecordProj {
    pub term: Box<EvalContext>,
    pub label: Label,
}

impl Eval for RecordProj {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        let ctx: EvalContext = RecordProjBeta {
            record: val,
            label: self.label,
        }
        .into();
        ctx.eval()
    }
}

impl From<RecordProj> for CongruenceRule {
    fn from(proj: RecordProj) -> CongruenceRule {
        CongruenceRule::RecordProj(proj)
    }
}

impl From<RecordProj> for EvalContext {
    fn from(proj: RecordProj) -> EvalContext {
        EvalContext::Congruence(proj.into())
    }
}

#[cfg(test)]
mod recproj_tests {
    use super::{Eval, RecordProj};
    use crate::{
        eval_context::{computation::HeadList, congruence::Record, Value},
        syntax::{Succ, Zero},
        types::Type,
    };
    use std::collections::HashMap;

    #[test]
    fn eval_proj() {
        let result = RecordProj {
            label: "label2".to_owned(),
            term: Box::new(
                Record {
                    vals: HashMap::from([
                        ("label1".to_owned(), Value::Zero),
                        ("label2".to_owned(), Value::True),
                    ]),
                    label: "label3".to_owned(),
                    next: Box::new(
                        HeadList {
                            list: Value::Cons {
                                fst: Box::new(Value::Zero),
                                rst: Box::new(Value::Nil {
                                    inner_type: Type::Nat,
                                }),
                                inner_type: Type::Nat,
                            },
                        }
                        .into(),
                    ),
                    rest: HashMap::from([(
                        "label4".to_owned(),
                        Succ {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                    )]),
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
