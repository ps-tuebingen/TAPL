use super::{to_kind_err, Env};
use crate::syntax::{kinds::Kind, types::OpApp};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for OpApp {
    type Type = Kind;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fun_kind = self.fun.check(&mut env.clone())?;
        let (from_kind, to_kind) = fun_kind.as_arrow().map_err(to_kind_err)?;
        let arg_kind = self.arg.check(env)?;
        from_kind.check_equal(&arg_kind).map_err(to_kind_err)?;
        Ok(to_kind)
    }
}
