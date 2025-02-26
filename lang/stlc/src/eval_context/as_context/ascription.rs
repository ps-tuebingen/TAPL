use super::{super::congruence, AsContext, Error, EvalContext};
use crate::syntax::Ascribe;

impl AsContext for Ascribe {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.term).try_into() {
            Ok(val) => Ok(EvalContext::Value(val)),
            Err(_) => {
                let inner = (*self.term).to_context()?;
                Ok(congruence::Ascribe {
                    term: Box::new(inner),
                }
                .into())
            }
        }
    }
}

#[cfg(test)]
mod ascribe_tests {
    use super::{AsContext, Ascribe, EvalContext};
    use crate::{
        eval_context::{computation::SuccPred, congruence, Value},
        syntax::{Pred, Succ, Zero},
        types::Type,
    };

    #[test]
    fn ctx_val() {
        let result = Ascribe {
            term: Box::new(Zero.into()),
            ty: Type::Nat,
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Zero);
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_cong() {
        let result = Ascribe {
            term: Box::new(
                Succ {
                    term: Box::new(
                        Pred {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            ty: Type::Nat,
        }
        .to_context()
        .unwrap();
        let expected = congruence::Ascribe {
            term: Box::new(SuccPred { val: Value::Zero }.into()),
        }
        .into();
        assert_eq!(result, expected)
    }
}
