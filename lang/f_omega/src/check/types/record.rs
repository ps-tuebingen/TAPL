use super::{to_kind_err, Env};
use crate::syntax::{kinds::Kind, types::RecTy};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for RecTy {
    type Type = Kind;
    type Err = Error;
    type Env = &'a Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        for (_, ty) in self.records.iter() {
            let ty_kind = ty.check(&mut env.clone())?;
            ty_kind.check_equal(&Kind::Star).map_err(to_kind_err)?;
        }
        Ok(Kind::Star)
    }
}
