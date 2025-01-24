use crate::{
    check::{Check, Env},
    errors::{Error, ErrorKind},
    syntax::{kinds::Kind, types::RecordTy},
};

impl Check for RecordTy {
    type Target = Kind;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
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
