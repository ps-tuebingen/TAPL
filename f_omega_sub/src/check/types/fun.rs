use crate::{
    check::{Check, Env},
    errors::Error,
    syntax::{kinds::Kind, types::Fun},
};

impl Check for Fun {
    type Target = Kind;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        let from_kind = self.from.check(&mut env.clone())?;
        from_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::kind(knd, self))?;
        let to_kind = self.to.check(env)?;
        to_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::kind(knd, self))?;
        Ok(Kind::Star)
    }
}
