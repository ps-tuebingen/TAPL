use crate::{
    check::Env,
    errors::{Error, ErrorKind},
    syntax::{kinds::Kind, types::RecordTy},
};
use common::Typecheck;

impl<'a> Typecheck<'a> for RecordTy {
    type Type = Kind;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let kinds = self
            .records
            .values()
            .map(|ty| ty.check(&mut env.clone()))
            .collect::<Result<Vec<Kind>, Error>>()?;
        kinds
            .iter()
            .map(|knd| knd.check_equal(&Kind::Star))
            .collect::<Result<Vec<()>, ErrorKind>>()
            .map_err(|knd| Error::kind(knd, self))?;
        Ok(Kind::Star)
    }
}
