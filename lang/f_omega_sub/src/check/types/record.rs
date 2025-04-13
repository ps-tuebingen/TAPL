use super::to_kind_err;
use crate::{
    check::Env,
    syntax::{kinds::Kind, types::RecordTy},
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for RecordTy {
    type Type = Kind;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let kinds = self
            .records
            .values()
            .map(|ty| ty.check(&mut env.clone()))
            .collect::<Result<Vec<Kind>, Error>>()?;
        kinds
            .iter()
            .map(|knd| knd.check_equal(&Kind::Star))
            .collect::<Result<Vec<()>, ErrorKind>>()
            .map_err(to_kind_err)?;
        Ok(Kind::Star)
    }
}
