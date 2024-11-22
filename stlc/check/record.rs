use super::{Check, TypingEnv};
use crate::{
    terms::syntax::{Record, RecordProj},
    types::Type,
};
use std::collections::HashMap;

impl Check for Record {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let mut tys = HashMap::new();
        for (label, term) in self.records.iter() {
            let ty = term.check_local(env)?;
            tys.insert(label.clone(), ty);
        }
        Some(Type::Record(tys))
    }
}

impl Check for RecordProj {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let rec_ty = self.record.check(env)?;
        if let Type::Record(tys) = rec_ty {
            tys.get(&self.label).cloned()
        } else {
            None
        }
    }
}
