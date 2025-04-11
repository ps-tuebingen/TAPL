use super::{to_eval_err, Value};
use crate::syntax::terms::{False, If, True};
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for True {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
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

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
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

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let cond_evaled = self.ifc.clone().eval(_env)?;
        match cond_evaled {
            Value::True => self.thent.eval(_env),
            Value::False => self.elset.eval(_env),
            v => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: v.to_string(),
                expected: "True or False".to_owned(),
            })),
        }
    }
}
