use super::{errors::Error, Value};
use crate::syntax::{False, If, True};
use common::Eval;

impl Eval<'_> for True {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::True)
    }
}

impl Eval<'_> for False {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::False)
    }
}

impl Eval<'_> for If {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
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
