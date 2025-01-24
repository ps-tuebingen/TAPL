use crate::{
    check::{CheckType, Env},
    errors::Error,
    syntax::{
        terms::Record,
        types::{RecTy, Type},
    },
};
use std::collections::HashMap;

impl CheckType for Record {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let mut recs = HashMap::new();
        for (label, t) in self.records.iter() {
            let t_ty = t.check_type(&mut env.clone())?;
            recs.insert(label.clone(), t_ty);
        }
        Ok(RecTy { records: recs }.into())
    }
}
