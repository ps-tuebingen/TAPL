use crate::{
    check::Env,
    errors::Error,
    syntax::{kinds::Kind, types::OpLambda},
};
use common::Typecheck;

impl<'a> Typecheck<'a> for OpLambda {
    type Type = Kind;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        env.add_tyvar(&self.var, &self.annot);
        let body_kind = self.body.check(env)?;
        Ok(Kind::Arrow(
            Box::new(self.annot.clone()),
            Box::new(body_kind),
        ))
    }
}
