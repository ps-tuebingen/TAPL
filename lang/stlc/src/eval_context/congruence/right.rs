use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::types::Type;

#[derive(Debug, PartialEq, Eq)]
pub struct Right {
    pub right_term: Box<EvalContext>,
    pub left_ty: Type,
}

impl Eval for Right {
    fn eval(self) -> Result<Value, Error> {
        let right_val = self.right_term.eval()?;
        Ok(Value::Right {
            right_term: Box::new(right_val),
            left_ty: self.left_ty,
        })
    }
}

impl From<Right> for CongruenceRule {
    fn from(right: Right) -> CongruenceRule {
        CongruenceRule::Right(right)
    }
}

impl From<Right> for EvalContext {
    fn from(right: Right) -> EvalContext {
        EvalContext::Congruence(right.into())
    }
}

#[cfg(test)]
mod right_tests {
    use super::{Eval, Right};
    use crate::{
        eval_context::{computation::IsZeroNum, Value},
        types::Type,
    };

    #[test]
    fn eval_right() {
        let result = Right {
            left_ty: Type::Nat,
            right_term: Box::new(IsZeroNum { num: Value::Zero }.into()),
        }
        .eval()
        .unwrap();
        let expected = Value::Right {
            left_ty: Type::Nat,
            right_term: Box::new(Value::True),
        };
        assert_eq!(result, expected)
    }
}
