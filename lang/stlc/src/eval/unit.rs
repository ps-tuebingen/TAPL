use super::{errors::Error, Eval, Value};
use crate::syntax::Unit;

impl Eval<'_> for Unit {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _: Self::Env) -> Result<Value, Error> {
        Ok(Value::Unit)
    }
}

#[cfg(test)]
mod unit_tests {
    use super::{Eval, Unit, Value};

    #[test]
    fn eval_unit() {
        let result = Unit.eval(Default::default()).unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }
}
