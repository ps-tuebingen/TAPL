use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::AsContext, syntax::Term, Label};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub vals: HashMap<Label, Value>,
    pub label: Label,
    pub next: Box<EvalContext>,
    pub rest: HashMap<Label, Term>,
}

impl Eval for Record {
    fn eval(self) -> Result<Value, Error> {
        let mut new_vals = self.vals;
        let next_val = self.next.eval()?;
        new_vals.insert(self.label, next_val);
        match self.rest.keys().next().cloned() {
            None => Ok(Value::Record(new_vals)),
            Some(next_label) => {
                let mut new_rest = self.rest;
                let next = new_rest.remove(&next_label).unwrap();
                let inner: EvalContext = next.to_context()?;
                let ctx: EvalContext = Record {
                    vals: new_vals,
                    label: next_label,
                    next: Box::new(inner),
                    rest: new_rest,
                }
                .into();
                ctx.eval()
            }
        }
    }
}

impl From<Record> for CongruenceRule {
    fn from(rec: Record) -> CongruenceRule {
        CongruenceRule::Record(rec)
    }
}

impl From<Record> for EvalContext {
    fn from(rec: Record) -> EvalContext {
        EvalContext::Congruence(rec.into())
    }
}

#[cfg(test)]
mod record_tests {
    use super::{Eval, Record};
    use crate::{
        eval_context::{computation::SuccPred, Value},
        syntax::{Cons, Head, Nil, Zero},
        types::Type,
    };
    use std::collections::HashMap;

    #[test]
    fn eval_record() {
        let result = Record {
            vals: HashMap::from([
                ("label1".to_owned(), Value::True),
                ("label2".to_owned(), Value::Zero),
            ]),
            label: "label3".to_owned(),
            next: Box::new(SuccPred { val: Value::Zero }.into()),
            rest: HashMap::from([(
                "label4".to_owned(),
                Head {
                    list: Box::new(
                        Cons {
                            fst: Box::new(Zero.into()),
                            rst: Box::new(
                                Nil {
                                    inner_type: Type::Nat,
                                }
                                .into(),
                            ),
                            inner_type: Type::Nat,
                        }
                        .into(),
                    ),
                    inner_type: Type::Nat,
                }
                .into(),
            )]),
        }
        .eval()
        .unwrap();
        let expected = Value::Record(HashMap::from([
            ("label1".to_owned(), Value::True),
            ("label2".to_owned(), Value::Zero),
            ("label3".to_owned(), Value::Zero),
            ("label4".to_owned(), Value::Zero),
        ]));
        assert_eq!(result, expected)
    }
}
