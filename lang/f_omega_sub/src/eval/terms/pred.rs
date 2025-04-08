use super::{Env, Value};
use crate::{
    errors::{Error, ErrorKind},
    syntax::terms::Pred,
};
use common::Eval;

impl<'a> Eval<'a> for Pred {
    type Value = Value;
    type Error = Error;
    type Env = &'a mut Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Error> {
        let val = self.term.clone().eval(env)?;
        match val {
            Value::Zero => Ok(Value::Pred {
                term: Box::new(Value::Zero),
            }),
            Value::Pred { term } => Ok(Value::Pred {
                term: Box::new(Value::Pred { term }),
            }),
            Value::Succ { term } => Ok(*term),
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
