use super::{to_check_err, Env};
use crate::syntax::{
    kinds::Kind,
    terms::Lambda,
    types::{Fun, Type},
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Lambda {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let annot_kind = self.annot.check(&mut env.clone())?;
        annot_kind.check_equal(&Kind::Star).map_err(to_check_err)?;
        env.add_var(&self.var, &self.annot);
        let body_ty = self.body.check(env)?;
        Ok(Fun {
            from: Box::new(self.annot.clone()),
            to: Box::new(body_ty),
        }
        .into())
    }
}
