use crate::{
    check::{Check, Env},
    errors::Error,
    syntax::{kinds::Kind, types::OpApp},
};

impl Check for OpApp {
    type Target = Kind;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        let fun_kind = self.fun.check(&mut env.clone())?;
        let (left, right) = fun_kind.as_arrow().map_err(|knd| Error::kind(knd, self))?;
        let arg_kind = self.arg.check(env)?;
        left.check_equal(&arg_kind)
            .map_err(|knd| Error::kind(knd, self))?;
        Ok(right)
    }
}
