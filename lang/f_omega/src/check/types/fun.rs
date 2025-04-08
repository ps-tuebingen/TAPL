use crate::{
    check::Env,
    errors::Error,
    syntax::{kinds::Kind, types::Fun},
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Fun {
    type Type = Kind;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let from_kind = self.from.check(&mut env.clone())?;
        from_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::kinding(knd, self))?;
        let to_kind = self.to.check(env)?;
        to_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::kinding(knd, self))?;
        Ok(Kind::Star)
    }
}
