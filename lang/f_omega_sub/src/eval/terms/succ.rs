use crate::{
    errors::{Error, ErrorKind},
    eval::{Env, Eval, Value},
    syntax::terms::Succ,
};

impl Eval for Succ {
    type Target = Value;
    fn eval(self, env: &mut Env) -> Result<Self::Target, Error> {
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
