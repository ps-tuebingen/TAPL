use crate::{
    errors::{Error, ErrorKind},
    eval::{Env, Eval, Value},
    syntax::terms::RecordProj,
};

impl Eval for RecordProj {
    type Target = Value;
    fn eval(self, env: &mut Env) -> Result<Self::Target, Error> {
        let val = self.term.clone().eval(env)?;
        let rec = val.as_rec().map_err(|knd| Error::eval(knd, self.clone()))?;
        rec.get(&self.label).cloned().ok_or(Error::eval(
            ErrorKind::UndefinedLabel(self.label.clone()),
            self,
        ))
    }
}
