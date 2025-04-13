use crate::{
    check::Env,
    syntax::{
        terms::Record,
        types::{RecordTy, Type},
    },
};
use common::{errors::Error, Eval, Typecheck};
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
            let ty = term.check(&mut env.clone())?.eval(&mut env.clone())?;
            tys.insert(label.clone(), ty);
        }

        Ok(RecordTy { records: tys }.into())
    }
}
