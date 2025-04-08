use crate::{
    check::Env,
    errors::{Error, ErrorKind},
    syntax::{terms::RecordProj, types::Type},
};
use common::Eval;
use common::Typecheck;

impl<'a> Typecheck<'a> for RecordProj {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let rec_ty = self.term.check(&mut env.clone())?.eval(env)?;
        let rec = rec_ty.as_rec().map_err(|knd| Error::check(knd, self))?;
        rec.records
            .get(&self.label)
            .ok_or(Error::check(
                ErrorKind::UndefinedLabel(self.label.clone()),
                self,
            ))
            .cloned()
    }
}
