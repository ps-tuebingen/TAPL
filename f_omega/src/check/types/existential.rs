use crate::{
    check::{CheckKind, Env},
    errors::Error,
    syntax::{kinds::Kind, types::Existential},
};

impl CheckKind for Existential {
    fn check_kind(&self, env: &mut Env) -> Result<Kind, Error> {
        env.add_tyvar(&self.ty_var, &self.kind);
        let ty_kind = self.ty.check_kind(env)?;
        ty_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::kinding(knd, self))?;
        Ok(Kind::Star)
    }
}
