use crate::{
    check::Env,
    errors::Error,
    syntax::{kinds::Kind, types::Universal},
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Universal {
    type Type = Kind;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        env.add_tyvar(&self.var, &self.sup_ty)?;
        let ty_kind = self.ty.check(env)?;
        ty_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::kind(knd, self))?;
        Ok(Kind::Star)
    }
}
