use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::computation::IfBool, syntax::Term};

#[derive(Debug, PartialEq, Eq)]
pub struct If {
    pub ifc: Box<EvalContext>,
    pub thenc: Term,
    pub elsec: Term,
}

impl Eval for If {
    fn eval(self) -> Result<Value, Error> {
        let val = self.ifc.eval()?;
        let ctx: EvalContext = IfBool {
            ifc: val,
            thenc: self.thenc,
            elsec: self.elsec,
        }
        .into();
        ctx.eval()
    }
}

impl From<If> for CongruenceRule {
    fn from(ift: If) -> CongruenceRule {
        CongruenceRule::If(ift)
    }
}

impl From<If> for EvalContext {
    fn from(ift: If) -> EvalContext {
        EvalContext::Congruence(ift.into())
    }
}

#[cfg(test)]
mod ift_tests {
    use super::{Eval, If};
    use crate::{
        eval_context::{computation::IsZeroNum, Value},
        syntax::{False, True},
    };

    #[test]
    fn eval_if() {
        let result = If {
            ifc: Box::new(IsZeroNum { num: Value::Zero }.into()),
            thenc: True.into(),
            elsec: False.into(),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
