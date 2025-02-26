use super::{ComputationRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct ProjBeta {
    pub tup: Value,
    pub ind: usize,
}

impl Eval for ProjBeta {
    fn eval(self) -> Result<Value, Error> {
        if let Value::Tup(vals) = self.tup {
            vals.get(self.ind)
                .cloned()
                .ok_or(Error::ProjectionOutOfBounds {
                    expected: self.ind,
                    found: vals.len(),
                })
        } else {
            Err(Error::BadValue { val: self.tup })
        }
    }
}

impl From<ProjBeta> for ComputationRule {
    fn from(proj: ProjBeta) -> ComputationRule {
        ComputationRule::ProjBeta(proj)
    }
}

impl From<ProjBeta> for EvalContext {
    fn from(proj: ProjBeta) -> EvalContext {
        EvalContext::Computation(proj.into())
    }
}

#[cfg(test)]
mod projbeta_tests {
    use super::{Eval, ProjBeta};
    use crate::{eval_context::Value, types::Type};

    #[test]
    fn eval_projbeta() {
        let result = ProjBeta {
            tup: Value::Tup(vec![
                Value::True,
                Value::False,
                Value::Nil {
                    inner_type: Type::Nat,
                },
            ]),
            ind: 1,
        }
        .eval()
        .unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }
}
