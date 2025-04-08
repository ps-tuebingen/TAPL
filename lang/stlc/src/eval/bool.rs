use super::{errors::Error, Value};
use crate::syntax::{False, If, True};
use common::Eval;

impl Eval for True {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _: &mut Self::Env) -> Result<Value, Error> {
        Ok(Value::True)
    }
}

impl Eval for False {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _: &mut Self::Env) -> Result<Value, Error> {
        Ok(Value::False)
    }
}

impl Eval for If {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: &mut Self::Env) -> Result<Value, Error> {
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
        let result = True.eval(&mut Default::default()).unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_false() {
        let result = False.eval(&mut Default::default()).unwrap();
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
        .eval(&mut Default::default())
        .unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }
}
