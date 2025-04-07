use super::{Eval, Value};
use crate::errors::{Error, ErrorKind};
use crate::syntax::terms::{False, If, True};

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
        let cond_evaled = self.ifc.clone().eval()?;
        match cond_evaled {
            Value::True => self.thent.eval(),
            Value::False => self.elset.eval(),
            v => Err(Error::eval(
                ErrorKind::BadValue {
                    found: v,
                    expected: "True or False".to_owned(),
                },
                &self,
            )),
        }
    }
}
