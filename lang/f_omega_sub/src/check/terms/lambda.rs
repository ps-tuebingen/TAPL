use crate::{
    check::{Check, Env},
    errors::Error,
    syntax::{
        kinds::Kind,
        terms::Lambda,
        types::{Fun, Type},
    },
};
use common::Eval;
impl Check for Lambda {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        let annot_kind = self.annot.check(&mut env.clone())?;
        annot_kind
            .check_equal(&Kind::Star)
            .map_err(|err| Error::check(err, self))?;
        let annot_evaled = self.annot.clone().eval(&mut env.clone())?;
        env.add_var(&self.var, &annot_evaled);
        let body_ty = self.body.check(&mut env.clone())?.eval(env)?;
        Ok(Fun::new(self.annot.clone(), body_ty).into())
    }
}
