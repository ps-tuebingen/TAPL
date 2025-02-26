use crate::{
    check::{CheckKind, Env},
    errors::Error,
    syntax::{kinds::Kind, types::OpApp},
};

impl CheckKind for OpApp {
    fn check_kind(&self, env: &mut Env) -> Result<Kind, Error> {
        let fun_kind = self.fun.check_kind(&mut env.clone())?;
        let (from_kind, to_kind) = fun_kind
            .as_arrow()
            .map_err(|knd| Error::kinding(knd, self))?;
        let arg_kind = self.arg.check_kind(env)?;
        from_kind
            .check_equal(&arg_kind)
            .map_err(|knd| Error::kinding(knd, self))?;
        Ok(to_kind)
    }
}
