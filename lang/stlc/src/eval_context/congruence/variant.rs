use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::types::Type;
use common::Label;

#[derive(Debug, PartialEq, Eq)]
pub struct Variant {
    pub term: Box<EvalContext>,
    pub label: Label,
    pub ty: Type,
}

impl Eval for Variant {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        Ok(Value::Variant {
            label: self.label,
            ty: self.ty,
            val: Box::new(val),
        })
    }
}

impl From<Variant> for CongruenceRule {
    fn from(var: Variant) -> CongruenceRule {
        CongruenceRule::Variant(var)
    }
}

impl From<Variant> for EvalContext {
    fn from(var: Variant) -> EvalContext {
        EvalContext::Congruence(var.into())
    }
}

#[cfg(test)]
mod variant_tests {
    use super::{Eval, Variant};
    use crate::{
        eval_context::{computation::SuccPred, Value},
        types::Type,
    };
    use std::collections::HashMap;

    fn example_ty() -> Type {
        Type::Variant(HashMap::from([
            ("label".to_owned(), Type::Nat),
            ("other".to_owned(), Type::Bool),
        ]))
    }

    #[test]
    fn eval_variant() {
        let result = Variant {
            label: "label".to_owned(),
            term: Box::new(SuccPred { val: Value::Zero }.into()),
            ty: example_ty(),
        }
        .eval()
        .unwrap();
        let expected = Value::Variant {
            label: "label".to_owned(),
            val: Box::new(Value::Zero),
            ty: example_ty(),
        };
        assert_eq!(result, expected)
    }
}
