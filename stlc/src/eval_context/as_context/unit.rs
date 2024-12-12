use super::{AsContext, Error, EvalContext};
use crate::{eval_context::Value, syntax::Unit};

impl AsContext for Unit {
    fn to_context(self) -> Result<EvalContext, Error> {
        Ok(EvalContext::Value(Value::Unit))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::{AsContext, EvalContext, Unit};
    use crate::eval_context::Value;

    #[test]
    fn ctx_unit() {
        let result = Unit.to_context().unwrap();
        let expected = EvalContext::Value(Value::Unit);
        assert_eq!(result, expected)
    }
}
