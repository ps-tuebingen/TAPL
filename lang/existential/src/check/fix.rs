use super::{Env, Error, ErrorKind};
use crate::{terms::Fix, types::Type};
use common::Typecheck;

impl<'a> Typecheck<'a> for Fix {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner_ty = self.term.check(env)?;
        let (from, to) = inner_ty.as_fun().map_err(|knd| Error::check(knd, self))?;
        if from == to {
            Ok(from)
        } else {
            Err(Error::check(
                ErrorKind::ty_mismatch(&from, &to.to_string()),
                self,
            ))
        }
    }
}
