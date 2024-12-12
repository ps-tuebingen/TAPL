use super::{ComputationRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::AsContext, syntax::Term};

#[derive(Debug, PartialEq, Eq)]
pub struct IfBool {
    pub ifc: Value,
    pub thenc: Term,
    pub elsec: Term,
}

impl Eval for IfBool {
    fn eval(self) -> Result<Value, Error> {
        match self.ifc {
            Value::True => {
                let ctx: EvalContext = self.thenc.to_context()?;
                ctx.eval()
            }
            Value::False => {
                let ctx: EvalContext = self.elsec.to_context()?;
                ctx.eval()
            }
            val => Err(Error::BadValue { val }),
        }
    }
}

impl From<IfBool> for ComputationRule {
    fn from(ifb: IfBool) -> ComputationRule {
        ComputationRule::IfBool(ifb)
    }
}

impl From<IfBool> for EvalContext {
    fn from(ifb: IfBool) -> EvalContext {
        EvalContext::Computation(ifb.into())
    }
}

#[cfg(test)]
mod ifbool_tests {
    use super::{Eval, IfBool};
    use crate::{
        eval_context::Value,
        syntax::{False, True},
    };

    #[test]
    fn eval_ifbool_true() {
        let result = IfBool {
            ifc: Value::True,
            thenc: False.into(),
            elsec: True.into(),
        }
        .eval()
        .unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_ifbool_false() {
        let result = IfBool {
            ifc: Value::False,
            thenc: False.into(),
            elsec: True.into(),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
