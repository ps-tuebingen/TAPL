use super::Value;
use crate::{
    errors::{Error, ErrorKind},
    syntax::terms::RecordProj,
};
use common::Eval;

impl Eval<'_> for RecordProj {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let rec_val = self.term.clone().eval(_env)?;
        let recs = rec_val.as_rec().map_err(|knd| Error::eval(knd, &self))?;
        recs.get(&self.label).cloned().ok_or(Error::eval(
            ErrorKind::UndefinedLabel(self.label.clone()),
            &self,
        ))
    }
}
