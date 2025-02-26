use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::AsContext, syntax::Term};

#[derive(Debug, PartialEq, Eq)]
pub struct Tup {
    pub values: Vec<Value>,
    pub next: Box<EvalContext>,
    pub rest: Vec<Term>,
}

impl Eval for Tup {
    fn eval(self) -> Result<Value, Error> {
        let next_val = self.next.eval()?;
        let mut new_vals = self.values;
        new_vals.push(next_val);
        if self.rest.is_empty() {
            Ok(Value::Tup(new_vals))
        } else {
            let mut new_rest = self.rest;
            let next = new_rest.remove(0);
            let inner_ctx: EvalContext = next.to_context()?;
            let ctx: EvalContext = Tup {
                values: new_vals,
                next: Box::new(inner_ctx),
                rest: new_rest,
            }
            .into();
            ctx.eval()
        }
    }
}

impl From<Tup> for CongruenceRule {
    fn from(right: Tup) -> CongruenceRule {
        CongruenceRule::Tup(right)
    }
}

impl From<Tup> for EvalContext {
    fn from(right: Tup) -> EvalContext {
        EvalContext::Congruence(right.into())
    }
}

#[cfg(test)]
mod tup_tests {
    use super::{Eval, Tup};
    use crate::{
        eval_context::{computation::SuccPred, Value},
        syntax::Nil,
        types::Type,
    };

    #[test]
    fn eval_tup() {
        let result = Tup {
            values: vec![Value::True],
            next: Box::new(
                SuccPred {
                    val: Value::Succ(Box::new(Value::Zero)),
                }
                .into(),
            ),
            rest: vec![Nil {
                inner_type: Type::Bool,
            }
            .into()],
        }
        .eval()
        .unwrap();
        let expected = Value::Tup(vec![
            Value::True,
            Value::Succ(Box::new(Value::Zero)),
            Value::Nil {
                inner_type: Type::Bool,
            },
        ]);
        assert_eq!(result, expected)
    }
}
