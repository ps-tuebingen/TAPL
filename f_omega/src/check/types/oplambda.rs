use crate::{
    check::{CheckKind, Env},
    errors::Error,
    syntax::{kinds::Kind, types::OpLambda},
};

impl CheckKind for OpLambda {
    fn check_kind(&self, env: &mut Env) -> Result<Kind, Error> {
        env.add_tyvar(&self.var, &self.annot);
        let body_kind = self.body.check_kind(env)?;
        Ok(Kind::Arrow(
            Box::new(self.annot.clone()),
            Box::new(body_kind),
        ))
    }
}
