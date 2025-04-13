use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::computation::LetSubst, syntax::Term};
use common::Var;

#[derive(Debug, PartialEq, Eq)]
pub struct Let {
    pub var: Var,
    pub bound_term: Box<EvalContext>,
    pub in_term: Term,
}

impl Eval for Let {
    fn eval(self) -> Result<Value, Error> {
        let val = self.bound_term.eval()?;
        let ctx: EvalContext = LetSubst {
            var: self.var,
            bound_val: val,
            in_term: self.in_term,
        }
        .into();
        ctx.eval()
    }
}

impl From<Let> for CongruenceRule {
    fn from(lt: Let) -> CongruenceRule {
        CongruenceRule::Let(lt)
    }
}
impl From<Let> for EvalContext {
    fn from(lt: Let) -> EvalContext {
        EvalContext::Congruence(lt.into())
    }
}

#[cfg(test)]
mod let_tests {
    use super::{Eval, Let};
    use crate::eval_context::{computation::IsZeroNum, Value};

    #[test]
    fn eval_let() {
        let result = Let {
            var: "x".to_owned(),
            bound_term: Box::new(IsZeroNum { num: Value::Zero }.into()),
            in_term: "x".to_owned().into(),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
