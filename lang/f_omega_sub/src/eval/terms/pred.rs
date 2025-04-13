use super::{to_eval_err, Env, Value};
use crate::syntax::terms::Pred;
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl<'a> Eval<'a> for Pred {
    type Value = Value;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.clone().eval(env)?;
        match val {
            Value::Zero => Ok(Value::Pred {
                term: Box::new(Value::Zero),
            }),
            Value::Pred { term } => Ok(Value::Pred {
                term: Box::new(Value::Pred { term }),
            }),
            Value::Succ { term } => Ok(*term),
            _ => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: val.to_string(),
                expected: "Natural Number".to_owned(),
            })),
        }
    }
}
