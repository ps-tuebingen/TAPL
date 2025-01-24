use super::{CheckType, Env};
use crate::{
    check::CheckKind,
    errors::Error,
    syntax::{
        kinds::Kind,
        terms::Lambda,
        types::{Fun, Type},
    },
};

impl CheckType for Lambda {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let annot_kind = self.annot.check_kind(&mut env.clone())?;
        annot_kind
            .check_equal(&Kind::Star)
            .map_err(|knd| Error::check(knd, self))?;
        env.add_var(&self.var, &self.annot);
        let body_ty = self.body.check_type(env)?;
        Ok(Fun {
            from: Box::new(self.annot.clone()),
            to: Box::new(body_ty),
        }
        .into())
    }
}
