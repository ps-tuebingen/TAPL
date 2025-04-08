use crate::{
    check::{Check, Env},
    errors::{Error, ErrorKind},
    syntax::{terms::RecordProj, types::Type},
};
use common::Eval;

impl Check for RecordProj {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        let rec_ty = self.term.check(&mut env.clone())?.eval(env)?;
        let rec = rec_ty.as_rec().map_err(|knd| Error::check(knd, self))?;
        rec.records
            .get(&self.label)
            .ok_or(Error::check(
                ErrorKind::UndefinedLabel(self.label.clone()),
                self,
            ))
            .cloned()
    }
}
