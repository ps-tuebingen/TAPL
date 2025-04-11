use super::{to_check_err, Env};
use crate::{terms::Fix, types::Type};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Fix {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner_ty = self.term.check(env)?;
        let (from, to) = inner_ty.as_fun().map_err(to_check_err)?;
        if from == to {
            Ok(from)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: from.to_string(),
                expected: to.to_string(),
            }))
        }
    }
}
