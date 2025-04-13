use super::Value;
use crate::syntax::Unit;
use common::{errors::Error, Eval};

impl Eval<'_> for Unit {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _: Self::Env) -> Result<Self::Value, Error> {
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
