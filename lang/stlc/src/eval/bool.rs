use super::{errors::Error, Value};
use crate::syntax::{False, If, True};
use common::Eval;

impl<'a> Eval<'a> for True {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _: Self::Env) -> Result<Value, Error> {
        Ok(Value::True)
    }
}

impl<'a> Eval<'a> for False {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _: Self::Env) -> Result<Value, Error> {
        Ok(Value::False)
    }
}

impl<'a> Eval<'a> for If {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        let if_v = self.ifc.eval(env)?;
        match if_v {
            Value::True => self.thenc.eval(env),
            Value::False => self.elsec.eval(env),
            _ => Err(Error::BadValue { val: if_v }),
        }
    }
}

#[cfg(test)]
mod bool_tests {
    use super::{Eval, False, If, True, Value};

    #[test]
    fn eval_true() {
        let result = True.eval(Default::default()).unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_false() {
        let result = False.eval(Default::default()).unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_if() {
        let result = If {
            ifc: Box::new(True.into()),
            thenc: Box::new(False.into()),
            elsec: Box::new(True.into()),
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }
}
