use super::{Eval, Value};
use crate::{
    errors::{Error, ErrorKind},
    syntax::terms::RecordProj,
};

impl Eval for RecordProj {
    fn eval(self) -> Result<Value, Error> {
        let rec_val = self.term.clone().eval()?;
        let recs = rec_val.as_rec().map_err(|knd| Error::eval(knd, &self))?;
        recs.get(&self.label).cloned().ok_or(Error::eval(
            ErrorKind::UndefinedLabel(self.label.clone()),
            &self,
        ))
    }
}
