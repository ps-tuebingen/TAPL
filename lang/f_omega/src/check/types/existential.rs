use super::{to_kind_err, Env};
use crate::syntax::{kinds::Kind, types::Existential};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Existential {
    type Type = Kind;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        env.add_tyvar(&self.ty_var, &self.kind);
        let ty_kind = self.ty.check(env)?;
        ty_kind.check_equal(&Kind::Star).map_err(to_kind_err)?;
        Ok(Kind::Star)
    }
}
