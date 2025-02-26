use crate::{
    check::{Check, Env},
    errors::Error,
    eval::Eval,
    syntax::{
        terms::Record,
        types::{RecordTy, Type},
    },
};
use std::collections::HashMap;

impl Check for Record {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        let mut tys = HashMap::new();
        for (label, term) in self.records.iter() {
            let ty = term.check(&mut env.clone())?.eval(&mut env.clone())?;
            tys.insert(label.clone(), ty);
        }

        Ok(RecordTy { records: tys }.into())
    }
}
