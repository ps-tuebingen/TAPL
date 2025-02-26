use crate::{
    errors::{Error, ErrorKind},
    eval::{Env, Eval, Value},
    syntax::terms::Pred,
};

impl Eval for Pred {
    type Target = Value;
    fn eval(self, env: &mut Env) -> Result<Self::Target, Error> {
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
