use super::{Check, Env, Error, ErrorKind};
use crate::{terms::Fix, types::Type};

impl Check for Fix {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
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
