use super::{errors::Error, Check, TypingEnv};
use crate::{
    terms::syntax::{Record, RecordProj},
    types::Type,
};
use std::collections::HashMap;

impl Check for Record {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let mut tys = HashMap::new();
        for (label, term) in self.records.iter() {
            let ty = term.check_local(env)?;
            tys.insert(label.clone(), ty);
        }
        Ok(Type::Record(tys))
    }
}

impl Check for RecordProj {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let rec_ty = self.record.check(env)?;
        if let Type::Record(tys) = rec_ty {
            tys.get(&self.label)
                .ok_or(Error::UndefinedLabel {
                    label: self.label.clone(),
                })
                .cloned()
        } else {
            Err(Error::UnexpectedType {
                ty: rec_ty.clone(),
                term: self.clone().into(),
            })
        }
    }
}
