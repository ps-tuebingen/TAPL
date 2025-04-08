use crate::{
    check::Env,
    errors::Error,
    syntax::{kinds::Kind, types::Existential},
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Existential {
    type Type = Kind;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        env.add_tyvar(&self.ty_var, &self.kind);
        let ty_kind = self.ty.check(env)?;
        ty_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::kinding(knd, self))?;
        Ok(Kind::Star)
    }
}
