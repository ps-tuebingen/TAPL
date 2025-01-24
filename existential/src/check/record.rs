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
            let t_ty = term.check(&mut env.clone())?;
            tys.insert(label.clone(), t_ty);
        }
        Ok(Type::Record(tys))
    }
}

impl Check for RecordProj {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let rec_ty = self.term.check(env)?;
        let recs = rec_ty.as_rec().map_err(|knd| Error::check(knd, self))?;
        recs.get(&self.label)
            .ok_or(Error::check(ErrorKind::label(&self.label), self))
            .cloned()
    }
}
