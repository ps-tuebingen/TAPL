use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::computation::SumCaseRhs, syntax::Term, Var};

#[derive(Debug, PartialEq, Eq)]
pub struct SumCase {
    pub bound_term: Box<EvalContext>,
    pub left_var: Var,
    pub left_term: Term,
    pub right_var: Var,
    pub right_term: Term,
}

impl Eval for SumCase {
    fn eval(self) -> Result<Value, Error> {
        let bound_val = self.bound_term.eval()?;
        let ctx: EvalContext = SumCaseRhs {
            bound_val,
            left_var: self.left_var,
            left_term: self.left_term,
            right_var: self.right_var,
            right_term: self.right_term,
        }
        .into();
        ctx.eval()
    }
}

impl From<SumCase> for CongruenceRule {
    fn from(sum: SumCase) -> CongruenceRule {
        CongruenceRule::SumCase(sum)
    }
}

impl From<SumCase> for EvalContext {
    fn from(sum: SumCase) -> EvalContext {
        EvalContext::Congruence(sum.into())
    }
}

#[cfg(test)]
mod sumcase_tests {
    use super::{Eval, SumCase};
    use crate::{
        eval_context::{computation::SuccPred, congruence::Right, Value},
        syntax::IsZero,
        types::Type,
    };

    #[test]
    fn eval_sumcase() {
        let result = SumCase {
            bound_term: Box::new(
                Right {
                    left_ty: Type::Bool,
                    right_term: Box::new(SuccPred { val: Value::Zero }.into()),
                }
                .into(),
            ),
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
