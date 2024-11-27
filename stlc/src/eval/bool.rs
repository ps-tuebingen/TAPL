use super::{errors::Error, Eval, Value};
use crate::syntax::{False, If, True};
impl Eval for True {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::True)
    }
}

impl Eval for False {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::False)
    }
}

impl Eval for If {
    fn eval(self) -> Result<Value, Error> {
        let if_v = self.ifc.eval()?;
        match if_v {
            Value::True => self.thenc.eval(),
            Value::False => self.elsec.eval(),
            _ => Err(Error::BadValue { val: if_v }),
        }
    }
}

#[cfg(test)]
mod bool_tests {
    use super::{Eval, False, If, True, Value};

    #[test]
    fn eval_true() {
        let result = True.eval().unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_false() {
        let result = False.eval().unwrap();
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
        .eval()
        .unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }
}
