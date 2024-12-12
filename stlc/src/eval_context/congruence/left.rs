use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::types::Type;

#[derive(Debug, PartialEq, Eq)]
pub struct Left {
    pub left_term: Box<EvalContext>,
    pub right_ty: Type,
}

impl Eval for Left {
    fn eval(self) -> Result<Value, Error> {
        let left_val = self.left_term.eval()?;
        Ok(Value::Left {
            left_term: Box::new(left_val),
            right_ty: self.right_ty,
        })
    }
}

impl From<Left> for CongruenceRule {
    fn from(left: Left) -> CongruenceRule {
        CongruenceRule::Left(left)
    }
}

impl From<Left> for EvalContext {
    fn from(left: Left) -> EvalContext {
        EvalContext::Congruence(left.into())
    }
}

#[cfg(test)]
mod left_tests {
    use super::{Eval, Left};
    use crate::{
        eval_context::{computation::SuccPred, Value},
        types::Type,
    };

    #[test]
    fn eval_left() {
        let result = Left {
            left_term: Box::new(SuccPred { val: Value::Zero }.into()),
            right_ty: Type::Bool,
        }
        .eval()
        .unwrap();
        let expected = Value::Left {
            left_term: Box::new(Value::Zero),
            right_ty: Type::Bool,
        };
        assert_eq!(result, expected)
    }
}
