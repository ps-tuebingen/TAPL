use super::{errors::Error, Eval, Value};
use crate::syntax::Unit;

impl Eval for Unit {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Unit)
    }
}

#[cfg(test)]
mod unit_tests {
    use super::{Eval, Unit, Value};

    #[test]
    fn eval_unit() {
        let result = Unit.eval().unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }
}
