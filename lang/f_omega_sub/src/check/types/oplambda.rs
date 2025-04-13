use crate::{
    check::Env,
    syntax::{
        kinds::Kind,
        types::{OpLambda, Type},
    },
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for OpLambda {
    type Type = Kind;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        env.add_tyvar(&self.var, &Type::Top(self.annot.clone()))?;
        let body_kind = self.body.check(env)?;
        Ok(Kind::Arrow(
            Box::new(self.annot.clone()),
            Box::new(body_kind),
        ))
    }
}
