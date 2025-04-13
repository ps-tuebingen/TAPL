use super::{to_eval_err, Env, Value};
use crate::syntax::terms::Succ;
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl<'a> Eval<'a> for Succ {
    type Value = Value;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.clone().eval(env)?;
        match val {
            Value::Zero => Ok(Value::Succ {
                term: Box::new(Value::Zero),
            }),
            Value::Succ { term } => Ok(Value::Succ {
                term: Box::new(Value::Succ { term }),
            }),
            Value::Pred { term } => Ok(*term),
            _ => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: val.to_string(),
                expected: "Natural Number".to_owned(),
            })),
        }
    }
}
