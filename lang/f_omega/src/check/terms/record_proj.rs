use crate::{
    check::Env,
    errors::Error,
    syntax::{terms::RecordProj, types::Type},
};

use common::Typecheck;
impl<'a> Typecheck<'a> for RecordProj {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let rec_ty = self.term.check(env)?;
        let recs = rec_ty.as_rec().map_err(|knd| Error::check(knd, self))?;
        recs.lookup(&self.label)
            .map_err(|knd| Error::check(knd, self))
    }
}
