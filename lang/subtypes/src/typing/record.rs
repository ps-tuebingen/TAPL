use super::{errors::Error, TypingContext};
use crate::{
    syntax::{Projection, Record},
    types::Type,
};
use common::Typecheck;
use std::collections::HashMap;

impl<'a> Typecheck<'a> for Record {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let mut rec_ty = HashMap::new();
        for (label, term) in self.records.iter() {
            let ty = term.check(&mut env.clone())?;
            rec_ty.insert(label.clone(), ty);
        }
        Ok(Type::Record(rec_ty))
    }
}

impl<'a> Typecheck<'a> for Projection {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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
