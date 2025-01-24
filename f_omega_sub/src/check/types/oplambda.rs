use crate::{
    check::{Check, Env},
    errors::Error,
    syntax::{
        kinds::Kind,
        types::{OpLambda, Type},
    },
};

impl Check for OpLambda {
    type Target = Kind;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        env.add_tyvar(&self.var, &Type::Top(self.annot.clone()))?;
        let body_kind = self.body.check(env)?;
        Ok(Kind::Arrow(
            Box::new(self.annot.clone()),
            Box::new(body_kind),
        ))
    }
}
