use crate::{
    check::{CheckKind, Env},
    errors::Error,
    syntax::{kinds::Kind, types::Fun},
};

impl CheckKind for Fun {
    fn check_kind(&self, env: &mut Env) -> Result<Kind, Error> {
        let from_kind = self.from.check_kind(&mut env.clone())?;
        from_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::kinding(knd, self))?;
        let to_kind = self.to.check_kind(env)?;
        to_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::kinding(knd, self))?;
        Ok(Kind::Star)
    }
}
