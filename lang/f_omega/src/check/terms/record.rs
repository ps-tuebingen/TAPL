use crate::{
    check::Env,
    syntax::{
        terms::Record,
        types::{RecTy, Type},
    },
};
use common::{errors::Error, Typecheck};
use std::collections::HashMap;

impl<'a> Typecheck<'a> for Record {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let mut recs = HashMap::new();
        for (label, t) in self.records.iter() {
            let t_ty = t.check(&mut env.clone())?;
            recs.insert(label.clone(), t_ty);
        }
        Ok(RecTy { records: recs }.into())
    }
}
