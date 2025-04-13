use super::{to_check_err, Env};
use crate::{
    syntax::{Projection, Record},
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
        let mut records = HashMap::new();
        for (lb, t) in self.records.iter() {
            let ty = t.check(&mut env.clone())?;
            records.insert(lb.clone(), ty);
        }
        Ok(Type::Record(records))
    }
}

impl<'a> Typecheck<'a> for Projection {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let rec_ty = self.record.check(env)?;
        let recs = if let Type::Var(v) = rec_ty {
            env.get_tyvar(&v)
                .map_err(to_check_err)?
                .as_record()
                .map_err(to_check_err)?
        } else {
            rec_ty.as_record().map_err(to_check_err)?
        };
        recs.get(&self.label)
            .cloned()
            .ok_or(ErrorKind::UndefinedLabel(self.label.clone()))
            .map_err(to_check_err)
    }
}
