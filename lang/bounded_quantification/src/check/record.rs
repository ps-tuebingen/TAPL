use super::{Check, Env, Error, ErrorKind};
use crate::{
    syntax::{Projection, Record},
    types::Type,
};
use std::collections::HashMap;

impl Check for Record {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let mut records = HashMap::new();
        for (lb, t) in self.records.iter() {
            let ty = t.check(&mut env.clone())?;
            records.insert(lb.clone(), ty);
        }
        Ok(Type::Record(records))
    }
}

impl Check for Projection {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let rec_ty = self.record.check(env)?;
        let recs = if let Type::Var(v) = rec_ty {
            env.get_tyvar(&v)
                .map_err(|knd| Error::check(knd, self))?
                .as_record()
                .map_err(|knd| Error::check(knd, self))?
        } else {
            rec_ty.as_record().map_err(|knd| Error::check(knd, self))?
        };
        recs.get(&self.label).cloned().ok_or(Error::check(
            ErrorKind::UndefinedLabel(self.label.clone()),
            self,
        ))
    }
}
