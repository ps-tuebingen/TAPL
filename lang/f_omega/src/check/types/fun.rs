use super::{to_kind_err, Env};
use crate::syntax::{kinds::Kind, types::Fun};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Fun {
    type Type = Kind;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let from_kind = self.from.check(&mut env.clone())?;
        from_kind.check_equal(&Kind::Star).map_err(to_kind_err)?;
        let to_kind = self.to.check(env)?;
        to_kind.check_equal(&Kind::Star).map_err(to_kind_err)?;
        Ok(Kind::Star)
    }
}
