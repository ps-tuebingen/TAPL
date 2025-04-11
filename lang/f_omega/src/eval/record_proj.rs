use super::{to_eval_err, Value};
use crate::syntax::terms::RecordProj;
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for RecordProj {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let rec_val = self.term.clone().eval(_env)?;
        let recs = rec_val.as_rec().map_err(to_eval_err)?;
        recs.get(&self.label)
            .cloned()
            .ok_or(to_eval_err(ErrorKind::UndefinedLabel(self.label.clone())))
    }
}
