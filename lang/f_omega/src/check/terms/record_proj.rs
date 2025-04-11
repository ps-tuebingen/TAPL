use super::{to_check_err, Env};
use crate::syntax::{terms::RecordProj, types::Type};
use common::{errors::Error, Typecheck};
impl<'a> Typecheck<'a> for RecordProj {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let rec_ty = self.term.check(env)?;
        let recs = rec_ty.as_rec().map_err(to_check_err)?;
        recs.lookup(&self.label).map_err(to_check_err)
    }
}
