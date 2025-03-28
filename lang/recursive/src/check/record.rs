use super::{Check, Env};
use crate::{
    errors::{Error, ErrorKind},
    terms::{Record, RecordProj},
    types::Type,
};
use std::collections::HashMap;

impl Check for Record {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let mut tys = HashMap::new();
        for (label, term) in self.records.iter() {
            let ty = term.check(&mut env.clone())?;
            tys.insert(label.clone(), ty);
        }
        Ok(Type::Record(tys))
    }
}

impl Check for RecordProj {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let rec_ty = self.record.check(env)?;

        let recs = rec_ty.as_record().map_err(|knd| Error::check(knd, self))?;
        let ty = recs.get(&self.label).ok_or(Error::check(
            ErrorKind::UndefinedLabel(self.label.clone()),
            self,
        ))?;
        Ok(ty.clone())
    }
}
