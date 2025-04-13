use super::to_check_err;
use crate::{
    check::Env,
    syntax::{terms::RecordProj, types::Type},
};
use common::{
    errors::{Error, ErrorKind},
    Eval, Typecheck,
};

impl<'a> Typecheck<'a> for RecordProj {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let rec_ty = self.term.check(&mut env.clone())?.eval(env)?;
        let rec = rec_ty.as_rec().map_err(to_check_err)?;
        rec.records
            .get(&self.label)
            .cloned()
            .ok_or(to_check_err(ErrorKind::UndefinedLabel(self.label.clone())))
    }
}
