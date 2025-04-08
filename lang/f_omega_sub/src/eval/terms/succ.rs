use super::{Env, Value};
use crate::{
    errors::{Error, ErrorKind},
    syntax::terms::Succ,
};
use common::Eval;

impl<'a> Eval<'a> for Succ {
    type Value = Value;
    type Error = Error;
    type Env = &'a mut Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Error> {
        let val = self.term.clone().eval(env)?;
        match val {
            Value::Zero => Ok(Value::Succ {
                term: Box::new(Value::Zero),
            }),
            Value::Succ { term } => Ok(Value::Succ {
                term: Box::new(Value::Succ { term }),
            }),
            Value::Pred { term } => Ok(*term),
            _ => Err(Error::eval(
                ErrorKind::BadValue {
                    found: val,
                    expected: "Natural Number".to_owned(),
                },
                self,
            )),
        }
    }
}
