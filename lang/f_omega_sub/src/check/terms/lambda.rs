use super::to_check_err;
use crate::{
    check::Env,
    syntax::{
        kinds::Kind,
        terms::Lambda,
        types::{Fun, Type},
    },
};
use common::{errors::Error, Eval, Typecheck};

impl<'a> Typecheck<'a> for Lambda {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let annot_kind = self.annot.check(&mut env.clone())?;
        annot_kind.check_equal(&Kind::Star).map_err(to_check_err)?;
        let annot_evaled = self.annot.clone().eval(&mut env.clone())?;
        env.add_var(&self.var, &annot_evaled);
        let body_ty = self.body.check(&mut env.clone())?.eval(env)?;
        Ok(Fun::new(self.annot.clone(), body_ty).into())
    }
}
