use super::{ComputationRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::AsContext, syntax::Term, traits::subst::Subst, Var};

#[derive(Debug, PartialEq, Eq)]
pub struct SumCaseRhs {
    pub bound_val: Value,
    pub left_var: Var,
    pub left_term: Term,
    pub right_var: Var,
    pub right_term: Term,
}

impl Eval for SumCaseRhs {
    fn eval(self) -> Result<Value, Error> {
        match self.bound_val {
            Value::Left {
                left_term,
                right_ty: _,
            } => {
                let t = self.left_term.subst(&self.left_var, (*left_term).into());
                let ctx: EvalContext = t.to_context()?;
                ctx.eval()
            }
            Value::Right {
                right_term,
                left_ty: _,
            } => {
                let t = self.right_term.subst(&self.right_var, (*right_term).into());
                let ctx: EvalContext = t.to_context()?;
                ctx.eval()
            }
            val => Err(Error::BadValue { val }),
        }
    }
}

impl From<SumCaseRhs> for ComputationRule {
    fn from(sum: SumCaseRhs) -> ComputationRule {
        ComputationRule::SumCaseRhs(sum)
    }
}

impl From<SumCaseRhs> for EvalContext {
    fn from(case: SumCaseRhs) -> EvalContext {
        EvalContext::Computation(case.into())
    }
}

#[cfg(test)]
mod sumcaserhs_tests {
    use super::{Eval, SumCaseRhs};
    use crate::{eval_context::Value, syntax::IsZero, types::Type};

    #[test]
    fn eval_sumcaserhs_left() {
        let result = SumCaseRhs {
            bound_val: Value::Left {
                left_term: Box::new(Value::False),
                right_ty: Type::Nat,
            },
            left_var: "x".to_owned(),
            left_term: "x".to_owned().into(),
            right_var: "y".to_owned(),
            right_term: IsZero {
                term: Box::new("y".to_owned().into()),
            }
            .into(),
        }
        .eval()
        .unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_somecaserhs_right() {
        let result = SumCaseRhs {
            bound_val: Value::Right {
                right_term: Box::new(Value::Zero),
                left_ty: Type::Bool,
            },
            left_var: "x".to_owned(),
            left_term: "x".to_owned().into(),
            right_var: "y".to_owned(),
            right_term: IsZero {
                term: Box::new("y".to_owned().into()),
            }
            .into(),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
