use crate::{
    check::Env,
    errors::Error,
    syntax::{kinds::Kind, types::OpApp},
};
use common::Typecheck;

impl<'a> Typecheck<'a> for OpApp {
    type Type = Kind;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fun_kind = self.fun.check(&mut env.clone())?;
        let (left, right) = fun_kind.as_arrow().map_err(|knd| Error::kind(knd, self))?;
        let arg_kind = self.arg.check(env)?;
        left.check_equal(&arg_kind)
            .map_err(|knd| Error::kind(knd, self))?;
        Ok(right)
    }
}
