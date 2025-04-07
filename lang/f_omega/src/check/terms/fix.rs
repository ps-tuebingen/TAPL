use super::{CheckType, Env};
use crate::{
    errors::Error,
    syntax::{terms::Fix, types::Type},
};

impl CheckType for Fix {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let inner_ty = self.term.check_type(env)?;
        let fun = inner_ty.as_fun().map_err(|knd| Error::check(knd, self))?;
        fun.from
            .check_equal(&fun.to)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(*fun.from)
    }
}
