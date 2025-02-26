use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::computation::SomeCaseRhs, syntax::Term, Var};

#[derive(Debug, PartialEq, Eq)]
pub struct SomeCase {
    pub bound_term: Box<EvalContext>,
    pub none_term: Term,
    pub some_var: Var,
    pub some_term: Term,
}

impl Eval for SomeCase {
    fn eval(self) -> Result<Value, Error> {
        let bound_val = self.bound_term.eval()?;
        let ctx: EvalContext = SomeCaseRhs {
            bound_val,
            none_term: self.none_term,
            some_var: self.some_var,
            some_term: self.some_term,
        }
        .into();
        ctx.eval()
    }
}

impl From<SomeCase> for CongruenceRule {
    fn from(case: SomeCase) -> CongruenceRule {
        CongruenceRule::SomeCase(case)
    }
}

impl From<SomeCase> for EvalContext {
    fn from(case: SomeCase) -> EvalContext {
        EvalContext::Congruence(case.into())
    }
}

#[cfg(test)]
mod somecase_tests {
    use super::{Eval, SomeCase};
    use crate::{
        eval_context::{computation::IsZeroNum, congruence::Something, Value},
        syntax::True,
    };

    #[test]
    fn eval_case() {
        let result = SomeCase {
            bound_term: Box::new(
                Something {
                    term: Box::new(
                        IsZeroNum {
                            num: Value::Succ(Box::new(Value::Zero)),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            none_term: True.into(),
            some_var: "x".to_owned(),
            some_term: "x".to_owned().into(),
        }
        .eval()
        .unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }
}
