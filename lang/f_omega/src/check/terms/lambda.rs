use super::Env;
use crate::{
    errors::Error,
    syntax::{
        kinds::Kind,
        terms::Lambda,
        types::{Fun, Type},
    },
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Lambda {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let annot_kind = self.annot.check(&mut env.clone())?;
        annot_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::check(knd, self))?;
        env.add_var(&self.var, &self.annot);
        let body_ty = self.body.check(env)?;
        Ok(Fun {
            from: Box::new(self.annot.clone()),
            to: Box::new(body_ty),
        }
        .into())
    }
}
