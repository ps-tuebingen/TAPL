use super::{errors::Error, Typecheck, TypingContext};
use crate::{
    syntax::{Projection, Record},
    types::Type,
};
use std::collections::HashMap;

impl Typecheck for Record {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        let mut rec_ty = HashMap::new();
        for (label, term) in self.records.iter() {
            let ty = term.check(&mut env.clone())?;
            rec_ty.insert(label.clone(), ty);
        }
        Ok(Type::Record(rec_ty))
    }
}

impl Typecheck for Projection {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        let rec_ty = self.record.check(env)?;
        let records = if let Type::Record(recs) = rec_ty {
            recs
        } else {
            return Err(Error::NoRecord(rec_ty));
        };
        records
            .get(&self.label)
            .cloned()
            .ok_or(Error::UndefinedLabel(self.label.clone()))
    }
}
