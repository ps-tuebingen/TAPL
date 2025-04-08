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
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fun_kind = self.fun.check(&mut env.clone())?;
        let (from_kind, to_kind) = fun_kind
            .as_arrow()
            .map_err(|knd| Error::kinding(knd, self))?;
        let arg_kind = self.arg.check(env)?;
        from_kind
            .check_equal(&arg_kind)
            .map_err(|knd| Error::kinding(knd, self))?;
        Ok(to_kind)
    }
}
