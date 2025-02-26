use crate::{
    check::{CheckKind, Env},
    errors::Error,
    syntax::{kinds::Kind, types::Universal},
};

impl CheckKind for Universal {
    fn check_kind(&self, env: &mut Env) -> Result<Kind, Error> {
        env.add_tyvar(&self.var, &self.kind);
        self.ty.check_kind(env)
    }
}
