use super::{AsContext, Error, EvalContext};
use crate::{
    eval_context::{computation::IfBool, congruence, Value},
    syntax::{False, If, True},
};

impl AsContext for True {
    fn to_context(self) -> Result<EvalContext, Error> {
        Ok(EvalContext::Value(Value::True))
    }
}
impl AsContext for False {
    fn to_context(self) -> Result<EvalContext, Error> {
        Ok(EvalContext::Value(Value::False))
    }
}

impl AsContext for If {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.ifc).try_into() {
            Ok(val) => Ok(IfBool {
                ifc: val,
                thenc: *self.thenc,
                elsec: *self.elsec,
            }
            .into()),
            Err(_) => {
                let if_ctx = (*self.ifc).to_context()?;
                Ok(congruence::If {
                    ifc: Box::new(if_ctx),
                    thenc: *self.thenc,
                    elsec: *self.elsec,
                }
                .into())
            }
        }
    }
}

#[cfg(test)]
mod bool_tests {
    use super::{AsContext, EvalContext, False, If, True, Value};
    use crate::{
        eval_context::{
            computation::{IfBool, IsZeroNum},
            congruence,
        },
        syntax::{IsZero, Zero},
    };

    #[test]
    fn ctx_true() {
        let result = True.to_context().unwrap();
        let expected = EvalContext::Value(Value::True);
        assert_eq!(result, expected)
    }
    #[test]
    fn ctx_false() {
        let result = False.to_context().unwrap();
        let expected = EvalContext::Value(Value::False);
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_if_comp() {
        let result = If {
            ifc: Box::new(True.into()),
            thenc: Box::new(False.into()),
            elsec: Box::new(False.into()),
        }
        .to_context()
        .unwrap();
        let expected = IfBool {
            ifc: Value::True,
            thenc: False.into(),
            elsec: False.into(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_if_cong() {
        let result = If {
            ifc: Box::new(
                IsZero {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
            thenc: Box::new(True.into()),
            elsec: Box::new(False.into()),
        }
        .to_context()
        .unwrap();
        let expected = congruence::If {
            ifc: Box::new(IsZeroNum { num: Value::Zero }.into()),
            thenc: True.into(),
            elsec: False.into(),
        }
        .into();
        assert_eq!(result, expected)
    }
}
