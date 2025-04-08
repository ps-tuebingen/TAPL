use super::{Env, Error, ErrorKind};
use crate::{
    syntax::{Projection, Record},
    types::Type,
};
use common::Typecheck;
use std::collections::HashMap;

impl<'a> Typecheck<'a> for Record {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let rec_ty = self.record.check(env)?;
        let recs = if let Type::Var(v) = rec_ty {
            env.get_tyvar(&v)
                .map_err(|knd| Error::check(knd, self))?
                .as_record()
                .map_err(|knd| Error::check(knd, self))?
        } else {
            rec_ty.as_record().map_err(|knd| Error::check(knd, self))?
        };
        recs.get(&self.label).cloned().ok_or(Error::check(
            ErrorKind::UndefinedLabel(self.label.clone()),
            self,
        ))
    }
}
