use super::{to_eval_err, Env, Value};
use crate::syntax::terms::RecordProj;
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl<'a> Eval<'a> for RecordProj {
    type Value = Value;
    type Err = Error;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.clone().eval(env)?;
        let rec = val.as_rec().map_err(to_eval_err)?;
        rec.get(&self.label)
            .cloned()
            .ok_or(to_eval_err(ErrorKind::UndefinedLabel(self.label.clone())))
    }
}
