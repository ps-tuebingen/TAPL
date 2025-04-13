use super::{to_check_err, Env};
use crate::{
    terms::{Record, RecordProj},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};
use std::collections::HashMap;

impl<'a> Typecheck<'a> for Record {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let mut tys = HashMap::new();
        for (label, term) in self.records.iter() {
            let ty = term.check(&mut env.clone())?;
            tys.insert(label.clone(), ty);
        }
        Ok(Type::Record(tys))
    }
}

impl<'a> Typecheck<'a> for RecordProj {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let rec_ty = self.record.check(env)?;

        let recs = rec_ty.as_record().map_err(to_check_err)?;
        let ty = recs
            .get(&self.label)
            .ok_or(to_check_err(ErrorKind::UndefinedLabel(self.label.clone())))?;
        Ok(ty.clone())
    }
}
