use crate::{
    check::{CheckType, Env},
    errors::Error,
    syntax::{terms::RecordProj, types::Type},
};

impl CheckType for RecordProj {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let rec_ty = self.term.check_type(env)?;
        let recs = rec_ty.as_rec().map_err(|knd| Error::check(knd, self))?;
        recs.lookup(&self.label)
            .map_err(|knd| Error::check(knd, self))
    }
}
