use super::to_kind_err;
use crate::{
    check::Env,
    syntax::{kinds::Kind, types::OpApp},
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for OpApp {
    type Type = Kind;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let fun_kind = self.fun.check(&mut env.clone())?;
        let (left, right) = fun_kind.as_arrow().map_err(to_kind_err)?;
        let arg_kind = self.arg.check(env)?;
        left.check_equal(&arg_kind).map_err(to_kind_err)?;
        Ok(right)
    }
}
